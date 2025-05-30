// Copyright (C) 2019-2025 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The compiler for Leo programs.
//!
//! The [`Compiler`] type compiles Leo programs into R1CS circuits.

use crate::CompilerOptions;

pub use leo_ast::Ast;
use leo_ast::{NodeBuilder, Program, Stub};
use leo_errors::{CompilerError, Result, emitter::Handler};
use leo_passes::*;
use leo_span::{Symbol, source_map::FileName, symbol::with_session_globals};

use snarkvm::prelude::Network;

use indexmap::{IndexMap, IndexSet};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

/// The primary entry point of the Leo compiler.
#[derive(Clone)]
pub struct Compiler<'a, N: Network> {
    /// The handler is used for error and warning emissions.
    handler: &'a Handler,
    /// The path to the main leo file.
    main_file_path: PathBuf,
    /// The path to where the compiler outputs all generated files.
    output_directory: PathBuf,
    /// The program name,
    pub program_name: String,
    /// The network name,
    pub network: String,
    /// The AST for the program.
    pub ast: Ast,
    /// Options configuring compilation.
    compiler_options: CompilerOptions,
    /// The `NodeCounter` used to generate sequentially increasing `NodeID`s.
    node_builder: NodeBuilder,
    /// The `Assigner` is used to construct (unique) assignment statements.
    assigner: Assigner,
    /// The type table.
    type_table: TypeTable,
    /// The stubs for imported programs. Produced by `Retriever` module.
    import_stubs: IndexMap<Symbol, Stub>,
    // Allows the compiler to be generic over the network.
    phantom: std::marker::PhantomData<N>,
}

impl<'a, N: Network> Compiler<'a, N> {
    /// Returns a new Leo compiler.
    pub fn new(
        program_name: String,
        network: String,
        handler: &'a Handler,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        compiler_options: Option<CompilerOptions>,
        import_stubs: IndexMap<Symbol, Stub>,
    ) -> Self {
        let node_builder = NodeBuilder::default();
        let assigner = Assigner::default();
        let type_table = TypeTable::default();
        Self {
            handler,
            main_file_path,
            output_directory,
            program_name,
            network,
            ast: Ast::new(Program::default()),
            compiler_options: compiler_options.unwrap_or_default(),
            node_builder,
            assigner,
            import_stubs,
            type_table,
            phantom: Default::default(),
        }
    }

    /// Returns a SHA256 checksum of the program file.
    pub fn checksum(&self) -> Result<String> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(format!("{hash:x}"))
    }

    /// Parses and stores a program file content from a string, constructs a syntax tree, and generates a program.
    pub fn parse_program_from_string(&mut self, program_string: &str, name: FileName) -> Result<()> {
        // Register the source (`program_string`) in the source map.
        let prg_sf = with_session_globals(|s| s.source_map.new_source(program_string, name));

        // Use the parser to construct the abstract syntax tree (ast).
        self.ast = leo_parser::parse_ast::<N>(self.handler, &self.node_builder, &prg_sf.src, prg_sf.start_pos)?;

        // If the program is imported, then check that the name of its program scope matches the file name.
        // Note that parsing enforces that there is exactly one program scope in a file.
        // TODO: Clean up check.
        let program_scope = self.ast.ast.program_scopes.values().next().unwrap();
        let program_scope_name = format!("{}", program_scope.program_id.name);
        if program_scope_name != self.program_name {
            return Err(CompilerError::program_scope_name_does_not_match(
                program_scope_name,
                self.program_name.clone(),
                program_scope.program_id.name.span,
            )
            .into());
        }

        if self.compiler_options.output.initial_ast {
            self.write_ast_to_json("initial_ast.json")?;
        }

        Ok(())
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let program_string = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(&self.main_file_path, e))?;

        self.parse_program_from_string(&program_string, FileName::Real(self.main_file_path.clone()))
    }

    /// Runs the symbol table pass.
    pub fn symbol_table_pass(&self) -> Result<SymbolTable> {
        let symbol_table = SymbolTableCreator::do_pass((&self.ast, self.handler))?;
        Ok(symbol_table)
    }

    /// Runs the type checker pass.
    pub fn type_checker_pass(&'a self, symbol_table: &mut SymbolTable) -> Result<(StructGraph, CallGraph)> {
        let (struct_graph, call_graph) =
            TypeChecker::do_pass((&self.ast, self.handler, symbol_table, &self.type_table, NetworkLimits {
                max_array_elements: N::MAX_ARRAY_ELEMENTS,
                max_mappings: N::MAX_MAPPINGS,
                max_functions: N::MAX_FUNCTIONS,
            }))?;
        Ok((struct_graph, call_graph))
    }

    /// Runs the static analysis pass.
    pub fn static_analysis_pass(&mut self, symbol_table: &SymbolTable) -> Result<()> {
        StaticAnalyzer::<N>::do_pass((
            &self.ast,
            self.handler,
            symbol_table,
            &self.type_table,
            self.compiler_options.build.conditional_block_max_depth,
            self.compiler_options.build.disable_conditional_branch_type_checking,
        ))
    }

    /// Run const propagation and loop unrolling until we hit a fixed point or find an error.
    pub fn const_propagation_and_unroll_loop(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        const LARGE_LOOP_BOUND: usize = 1024usize;

        for _ in 0..LARGE_LOOP_BOUND {
            let loop_unroll_output = self.loop_unrolling_pass(symbol_table)?;

            let const_prop_output = self.const_propagation_pass(symbol_table)?;

            if !const_prop_output.changed && !loop_unroll_output.loop_unrolled {
                // We've got a fixed point, so see if we have any errors.
                if let Some(not_evaluated_span) = const_prop_output.const_not_evaluated {
                    return Err(CompilerError::const_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_evaluated_span) = const_prop_output.array_index_not_evaluated {
                    return Err(CompilerError::array_index_not_evaluated(not_evaluated_span).into());
                }

                if let Some(not_unrolled_span) = loop_unroll_output.loop_not_unrolled {
                    return Err(CompilerError::loop_bounds_not_evaluated(not_unrolled_span).into());
                }

                if self.compiler_options.output.unrolled_ast {
                    self.write_ast_to_json("unrolled_ast.json")?;
                }

                return Ok(());
            }
        }

        // Note that it's challenging to write code in practice that demonstrates this error, because Leo code
        // with many nested loops or operations will blow the stack in the compiler before this bound is hit.
        Err(CompilerError::const_prop_unroll_many_loops(LARGE_LOOP_BOUND, Default::default()).into())
    }

    /// Runs the const propagation pass.
    pub fn const_propagation_pass(&mut self, symbol_table: &mut SymbolTable) -> Result<ConstPropagatorOutput> {
        let (ast, output) = ConstPropagator::do_pass((
            std::mem::take(&mut self.ast),
            self.handler,
            symbol_table,
            &self.type_table,
            &self.node_builder,
        ))?;

        self.ast = ast;

        Ok(output)
    }

    /// Runs the loop unrolling pass.
    pub fn loop_unrolling_pass(&mut self, symbol_table: &mut SymbolTable) -> Result<UnrollerOutput> {
        let (ast, output) = Unroller::do_pass((
            std::mem::take(&mut self.ast),
            self.handler,
            &self.node_builder,
            symbol_table,
            &self.type_table,
        ))?;

        self.ast = ast;

        Ok(output)
    }

    /// Runs the static single assignment pass.
    pub fn static_single_assignment_pass(&mut self, symbol_table: &SymbolTable) -> Result<()> {
        self.ast = StaticSingleAssigner::do_pass((
            std::mem::take(&mut self.ast),
            &self.node_builder,
            &self.assigner,
            symbol_table,
            &self.type_table,
        ))?;

        if self.compiler_options.output.ssa_ast {
            self.write_ast_to_json("ssa_ast.json")?;
        }

        Ok(())
    }

    /// Runs the flattening pass.
    pub fn flattening_pass(&mut self, symbol_table: &SymbolTable) -> Result<()> {
        self.ast = Flattener::do_pass((
            std::mem::take(&mut self.ast),
            symbol_table,
            &self.type_table,
            &self.node_builder,
            &self.assigner,
        ))?;

        if self.compiler_options.output.flattened_ast {
            self.write_ast_to_json("flattened_ast.json")?;
        }

        Ok(())
    }

    /// Runs the destructuring pass.
    pub fn destructuring_pass(&mut self) -> Result<()> {
        self.ast = Destructurer::do_pass((
            std::mem::take(&mut self.ast),
            &self.type_table,
            &self.node_builder,
            &self.assigner,
        ))?;

        if self.compiler_options.output.destructured_ast {
            self.write_ast_to_json("destructured_ast.json")?;
        }

        Ok(())
    }

    /// Runs the function inlining pass.
    pub fn function_inlining_pass(&mut self, call_graph: &CallGraph) -> Result<()> {
        let ast = FunctionInliner::do_pass((
            std::mem::take(&mut self.ast),
            &self.node_builder,
            call_graph,
            &self.assigner,
            &self.type_table,
        ))?;
        self.ast = ast;

        if self.compiler_options.output.inlined_ast {
            self.write_ast_to_json("inlined_ast.json")?;
        }

        Ok(())
    }

    /// Runs the dead code elimination pass.
    pub fn dead_code_elimination_pass(&mut self) -> Result<()> {
        if self.compiler_options.build.dce_enabled {
            self.ast = DeadCodeEliminator::do_pass((std::mem::take(&mut self.ast), &self.node_builder))?;
        }

        if self.compiler_options.output.dce_ast {
            self.write_ast_to_json("dce_ast.json")?;
        }

        Ok(())
    }

    /// Runs the code generation pass.
    pub fn code_generation_pass(
        &mut self,
        symbol_table: &SymbolTable,
        struct_graph: &StructGraph,
        call_graph: &CallGraph,
    ) -> Result<String> {
        CodeGenerator::do_pass((&self.ast, symbol_table, &self.type_table, struct_graph, call_graph, &self.ast.ast))
    }

    /// Runs the compiler stages.
    pub fn compiler_stages(&mut self) -> Result<(SymbolTable, StructGraph, CallGraph)> {
        let mut st = self.symbol_table_pass()?;

        let (struct_graph, call_graph) = self.type_checker_pass(&mut st)?;

        self.static_analysis_pass(&st)?;

        self.const_propagation_and_unroll_loop(&mut st)?;

        self.static_single_assignment_pass(&st)?;

        self.flattening_pass(&st)?;

        self.destructuring_pass()?;

        self.function_inlining_pass(&call_graph)?;

        self.dead_code_elimination_pass()?;

        Ok((st, struct_graph, call_graph))
    }

    /// Returns a compiled Leo program.
    pub fn compile(&mut self) -> Result<String> {
        // Parse the program.
        self.parse_program()?;
        // Copy the dependencies specified in `program.json` into the AST.
        self.add_import_stubs()?;
        // Run the intermediate compiler stages.
        let (symbol_table, struct_graph, call_graph) = self.compiler_stages()?;
        // Run code generation.
        let bytecode = self.code_generation_pass(&symbol_table, &struct_graph, &call_graph)?;
        Ok(bytecode)
    }

    /// Writes the AST to a JSON file.
    fn write_ast_to_json(&self, file_suffix: &str) -> Result<()> {
        // Remove `Span`s if they are not enabled.
        if self.compiler_options.output.ast_spans_enabled {
            self.ast.to_json_file(self.output_directory.clone(), &format!("{}.{file_suffix}", self.program_name))?;
        } else {
            self.ast.to_json_file_without_keys(
                self.output_directory.clone(),
                &format!("{}.{file_suffix}", self.program_name),
                &["_span", "span"],
            )?;
        }
        Ok(())
    }

    /// Merges the dependencies defined in `program.json` with the dependencies imported in `.leo` file
    pub fn add_import_stubs(&mut self) -> Result<()> {
        // Create a list of both the explicit dependencies specified in the `.leo` file, as well as the implicit ones derived from those dependencies.
        let (mut unexplored, mut explored): (IndexSet<Symbol>, IndexSet<Symbol>) =
            (self.ast.ast.imports.keys().cloned().collect(), IndexSet::new());
        while !unexplored.is_empty() {
            let mut current_dependencies: IndexSet<Symbol> = IndexSet::new();
            for program_name in unexplored.iter() {
                if let Some(stub) = self.import_stubs.get(program_name) {
                    // Add the program to the explored set
                    explored.insert(*program_name);
                    for dependency in stub.imports.iter() {
                        // If dependency is already explored then don't need to re-explore it
                        if explored.insert(dependency.name.name) {
                            current_dependencies.insert(dependency.name.name);
                        }
                    }
                } else {
                    return Err(CompilerError::imported_program_not_found(
                        self.program_name.clone(),
                        *program_name,
                        self.ast.ast.imports[program_name].1,
                    )
                    .into());
                }
            }

            // Create next batch to explore
            unexplored = current_dependencies;
        }

        // Combine the dependencies from `program.json` and `.leo` file while preserving the post-order
        self.ast.ast.stubs =
            self.import_stubs.clone().into_iter().filter(|(program_name, _)| explored.contains(program_name)).collect();
        Ok(())
    }
}
