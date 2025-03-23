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

// NOTE: If compiler passes are made optional, pass preconditions and invariants may not necessarily hold true.

#[derive(Clone, Default)]
pub struct CompilerOptions {
    /// Build options.
    pub build: BuildOptions,
    /// Output options.
    pub output: OutputOptions,
}

#[derive(Clone, Default)]
pub struct BuildOptions {
    /// Whether to enable dead code elimination.
    pub dce_enabled: bool,
    /// Max depth to type check nested conditionals.
    pub conditional_block_max_depth: usize,
    /// Whether to disable type checking for nested conditionals.
    pub disable_conditional_branch_type_checking: bool,
}

#[derive(Clone, Default)]
pub struct OutputOptions {
    /// Whether spans are enabled in the output ASTs.
    pub ast_spans_enabled: bool,
    /// If enabled writes the AST after parsing.
    pub initial_ast: bool,
    /// If enabled writes the AST after loop unrolling.
    pub unrolled_ast: bool,
    /// If enabled writes the AST after static single assignment.
    pub ssa_ast: bool,
    /// If enabled writes the AST after flattening.
    pub flattened_ast: bool,
    /// If enabled writes the AST after destructuring.
    pub destructured_ast: bool,
    /// If enabled writes the AST after inlining.
    pub inlined_ast: bool,
    /// If enabled writes the AST after dead code elimination.
    pub dce_ast: bool,
}
