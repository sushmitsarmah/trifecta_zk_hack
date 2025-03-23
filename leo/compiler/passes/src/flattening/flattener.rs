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

use crate::{Assigner, SymbolTable, TypeTable};

use leo_ast::{
    AccessExpression,
    ArrayAccess,
    ArrayExpression,
    ArrayType,
    BinaryExpression,
    BinaryOperation,
    Block,
    Composite,
    CompositeType,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IntegerType,
    Literal,
    Member,
    MemberAccess,
    Node,
    NodeBuilder,
    NonNegativeNumber,
    ReturnStatement,
    Statement,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    TupleType,
    Type,
    UnitExpression,
};
use leo_span::Symbol;

/// An expression representing a conditional to reach the current
/// point in the AST.
#[derive(Clone, Copy)]
pub enum Guard {
    /// An Unconstructed guard is one representing a single conditional
    /// on the stack of conditions.
    Unconstructed(Identifier),

    /// A Constructed guard is one which as been `And`ed with all previous
    /// conditions on the stack.
    ///
    /// We cache this so that we don't have to evaluate the same chain
    /// of conditions repeatedly.
    Constructed(Identifier),
}

#[derive(Clone, Copy)]
pub enum ReturnGuard {
    /// There were no conditionals on the path to this return statement.
    None,

    /// There was a chain of conditionals on the path to this return statement,
    /// and they are true iff this Identifier is true.
    Unconstructed(Identifier),

    /// There was a chain of conditionals on the path to this return statement.`
    Constructed {
        /// True iff the conditionals on the path to this return statement are true.
        plain: Identifier,

        /// True iff any of the guards to return statements so far encountered
        /// are true. We cache this to guard asserts against early returns.
        any_return: Identifier,
    },
}

impl Guard {
    fn identifier(self) -> Identifier {
        match self {
            Guard::Constructed(id) | Guard::Unconstructed(id) => id,
        }
    }
}

pub struct Flattener<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A mapping between node IDs and their types.
    pub(crate) type_table: &'a TypeTable,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// A struct used to construct (unique) assignment statements.
    pub(crate) assigner: &'a Assigner,

    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Guard>,

    /// A list containing tuples of guards and expressions associated `ReturnStatement`s.
    /// A guard is an expression that evaluates to true on the execution path of the `ReturnStatement`.
    /// Note that returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    /// Note that type checking guarantees that there is at most one return in a basic block.
    pub(crate) returns: Vec<(ReturnGuard, ReturnStatement)>,

    /// The program name.
    pub(crate) program: Option<Symbol>,
    /// Whether the function is an async function.
    pub(crate) is_async: bool,
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(
        symbol_table: &'a SymbolTable,
        type_table: &'a TypeTable,
        node_builder: &'a NodeBuilder,
        assigner: &'a Assigner,
    ) -> Self {
        Self {
            symbol_table,
            type_table,
            node_builder,
            assigner,
            condition_stack: Vec::new(),
            returns: Vec::new(),
            program: None,
            is_async: false,
        }
    }

    /// Construct an early return guard.
    ///
    /// That is, an Identifier assigned to a boolean that is true iff some early return was taken.
    pub(crate) fn construct_early_return_guard(&mut self) -> Option<(Identifier, Vec<Statement>)> {
        if self.returns.is_empty() {
            return None;
        }

        if self.returns.iter().any(|g| matches!(g.0, ReturnGuard::None)) {
            // There was a return with no conditions, so we should simple return True.
            let place = Identifier {
                name: self.assigner.unique_symbol("true", "$"),
                span: Default::default(),
                id: self.node_builder.next_id(),
            };
            let statement = self.simple_assign_statement(
                place,
                Expression::Literal(Literal::Boolean(true, Default::default(), self.node_builder.next_id())),
            );
            return Some((place, vec![statement]));
        }

        // All guards up to a certain point in the stack should be constructed.
        // Find the first unconstructed one.
        let start_i = (0..self.returns.len())
            .rev()
            .take_while(|&i| matches!(self.returns[i].0, ReturnGuard::Unconstructed(_)))
            .last()
            .unwrap_or(self.returns.len());

        let mut statements = Vec::with_capacity(self.returns.len() - start_i);

        for i in start_i..self.returns.len() {
            let ReturnGuard::Unconstructed(identifier) = self.returns[i].0 else {
                unreachable!("We assured above that all guards after the index are Unconstructed.");
            };
            if i == 0 {
                self.returns[i].0 = ReturnGuard::Constructed { plain: identifier, any_return: identifier };
                continue;
            }

            let ReturnGuard::Constructed { any_return: previous_identifier, .. } = self.returns[i - 1].0 else {
                unreachable!("We're always at an index where previous guards were Constructed.");
            };

            let identifier_expression = Expression::Identifier(identifier);
            let previous_expression = Expression::Identifier(previous_identifier);

            // Construct an Or of the two expressions.
            let binary = Expression::Binary(BinaryExpression {
                op: BinaryOperation::Or,
                left: Box::new(previous_expression),
                right: Box::new(identifier_expression),
                span: Default::default(),
                id: {
                    let id = self.node_builder.next_id();
                    self.type_table.insert(id, Type::Boolean);
                    id
                },
            });

            // Assign that Or to a new Identifier.
            let place = Identifier {
                name: self.assigner.unique_symbol("guard", "$"),
                span: Default::default(),
                id: self.node_builder.next_id(),
            };
            statements.push(self.simple_assign_statement(place, binary));

            // Make that assigned Identifier the constructed guard.
            self.returns[i].0 = ReturnGuard::Constructed { plain: identifier, any_return: place };
        }

        let ReturnGuard::Constructed { any_return, .. } = self.returns.last().unwrap().0 else {
            unreachable!("Above we made all guards Constructed.");
        };

        Some((any_return, statements))
    }

    /// Construct a guard from the current state of the condition stack.
    ///
    /// That is, a boolean expression which is true iff we've followed the branches
    /// that led to the current point in the Leo code.
    pub(crate) fn construct_guard(&mut self) -> Option<(Identifier, Vec<Statement>)> {
        if self.condition_stack.is_empty() {
            return None;
        }

        // All guards up to a certain point in the stack should be constructed.
        // Find the first unconstructed one. Start the search at the end so we
        // don't repeatedly traverse the whole stack with repeated calls to this
        // function.
        let start_i = (0..self.condition_stack.len())
            .rev()
            .take_while(|&i| matches!(self.condition_stack[i], Guard::Unconstructed(_)))
            .last()
            .unwrap_or(self.condition_stack.len());

        let mut statements = Vec::with_capacity(self.condition_stack.len() - start_i);

        for i in start_i..self.condition_stack.len() {
            let identifier = self.condition_stack[i].identifier();
            if i == 0 {
                self.condition_stack[0] = Guard::Constructed(identifier);
                continue;
            }

            let previous = self.condition_stack[i - 1].identifier();
            let identifier_expression = Expression::Identifier(identifier);
            let previous_expression = Expression::Identifier(previous);

            // Construct an And of the two expressions.
            let binary = Expression::Binary(BinaryExpression {
                op: BinaryOperation::And,
                left: Box::new(previous_expression),
                right: Box::new(identifier_expression),
                span: Default::default(),
                id: {
                    // Create a new node ID for the binary expression.
                    let id = self.node_builder.next_id();
                    // Set the type of the node ID.
                    self.type_table.insert(id, Type::Boolean);
                    id
                },
            });

            // Assign that And to a new Identifier.
            let place = Identifier {
                name: self.assigner.unique_symbol("guard", "$"),
                span: Default::default(),
                id: self.node_builder.next_id(),
            };
            statements.push(self.simple_assign_statement(place, binary));

            // Make that assigned Identifier the constructed guard.
            self.condition_stack[i] = Guard::Constructed(place);
        }

        Some((self.condition_stack.last().unwrap().identifier(), statements))
    }

    /// Fold guards and expressions into a single expression.
    /// Note that this function assumes that at least one guard is present.
    pub(crate) fn fold_guards(
        &mut self,
        prefix: &str,
        mut guards: Vec<(Option<Expression>, Expression)>,
    ) -> (Expression, Vec<Statement>) {
        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_expression) = guards.pop().unwrap();

        match last_expression {
            // If the expression is a unit expression, then return it directly.
            Expression::Unit(_) => (last_expression, Vec::new()),
            // Otherwise, fold the guards and expressions into a single expression.
            _ => {
                // Produce a chain of ternary expressions and assignments for the guards.
                let mut statements = Vec::with_capacity(guards.len());

                // Helper to construct and store ternary assignments. e.g `$ret$0 = $var$0 ? $var$1 : $var$2`
                let mut construct_ternary_assignment =
                    |guard: Expression, if_true: Expression, if_false: Expression| {
                        let place = Identifier {
                            name: self.assigner.unique_symbol(prefix, "$"),
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        };
                        let (value, stmts) = self.reconstruct_ternary(TernaryExpression {
                            id: {
                                // Create a new node ID for the ternary expression.
                                let id = self.node_builder.next_id();
                                // Get the type of the node ID.
                                let type_ = match self.type_table.get(&if_true.id()) {
                                    Some(type_) => type_,
                                    None => unreachable!("Type checking guarantees that all expressions have a type."),
                                };
                                // Set the type of the node ID.
                                self.type_table.insert(id, type_);
                                id
                            },
                            condition: Box::new(guard),
                            if_true: Box::new(if_true),
                            if_false: Box::new(if_false),
                            span: Default::default(),
                        });
                        statements.extend(stmts);

                        match &value {
                            // If the expression is a tuple, then use it directly.
                            // This must be done to ensure that intermediate tuple assignments are not created.
                            Expression::Tuple(_) => value,
                            // Otherwise, assign the expression to a variable and return the variable.
                            _ => {
                                statements.push(self.simple_assign_statement(place, value));
                                Expression::Identifier(place)
                            }
                        }
                    };

                let expression = guards.into_iter().rev().fold(last_expression, |acc, (guard, expr)| match guard {
                    None => unreachable!("All expressions except for the last one must have a guard."),
                    // Note that type checking guarantees that all expressions have the same type.
                    Some(guard) => construct_ternary_assignment(guard, expr, acc),
                });

                (expression, statements)
            }
        }
    }

    /// A wrapper around `assigner.unique_simple_assign_statement` that updates `self.structs`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.assigner.unique_symbol("$var", "$");
        // Construct the lhs of the assignment.
        let place = Identifier { name, span: Default::default(), id: self.node_builder.next_id() };
        // Construct the assignment statement.
        let statement = self.simple_assign_statement(place, expr);

        (place, statement)
    }

    /// A wrapper around `assigner.simple_assign_statement` that tracks the type of the lhs.
    pub(crate) fn simple_assign_statement(&mut self, lhs: Identifier, rhs: Expression) -> Statement {
        // Update the type table.
        let type_ = match self.type_table.get(&rhs.id()) {
            Some(type_) => type_,
            None => unreachable!("Type checking guarantees that all expressions have a type."),
        };
        self.type_table.insert(lhs.id(), type_);
        // Construct the statement.
        self.assigner.simple_assign_statement(lhs, rhs, self.node_builder.next_id())
    }

    /// Folds a list of return statements into a single return statement and adds the produced statements to the block.
    pub(crate) fn fold_returns(&mut self, block: &mut Block, returns: Vec<(Option<Expression>, ReturnStatement)>) {
        // If the list of returns is not empty, then fold them into a single return statement.
        if !returns.is_empty() {
            let mut return_expressions = Vec::with_capacity(returns.len());

            // Aggregate the return expressions and finalize arguments and their respective guards.
            for (guard, return_statement) in returns {
                return_expressions.push((guard.clone(), return_statement.expression));
            }

            // Fold the return expressions into a single expression.
            let (expression, stmts) = self.fold_guards("$ret", return_expressions);

            // Add all of the accumulated statements to the end of the block.
            block.statements.extend(stmts);

            // Add the `ReturnStatement` to the end of the block.
            block.statements.push(Statement::Return(ReturnStatement {
                expression,
                span: Default::default(),
                id: self.node_builder.next_id(),
            }));
        }
        // Otherwise, push a dummy return statement to the end of the block.
        else {
            block.statements.push(Statement::Return(ReturnStatement {
                expression: {
                    let id = self.node_builder.next_id();
                    Expression::Unit(UnitExpression { span: Default::default(), id })
                },
                span: Default::default(),
                id: self.node_builder.next_id(),
            }));
        }
    }

    pub(crate) fn ternary_array(
        &mut self,
        array: &ArrayType,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each array element, construct a new ternary expression.
        let elements = (0..array.length())
            .map(|i| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Array(ArrayAccess {
                        array: Box::new(Expression::Identifier(*first)),
                        index: Box::new(Expression::Literal(Literal::Integer(
                            IntegerType::U32,
                            i.to_string(),
                            Default::default(),
                            {
                                // Create a new node ID for the literal.
                                let id = self.node_builder.next_id();
                                // Set the type of the node ID.
                                self.type_table.insert(id, Type::Integer(IntegerType::U32));
                                id
                            },
                        ))),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, array.element_type().clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Array(ArrayAccess {
                        array: Box::new(Expression::Identifier(*second)),
                        index: Box::new(Expression::Literal(Literal::Integer(
                            IntegerType::U32,
                            i.to_string(),
                            Default::default(),
                            {
                                // Create a new node ID for the literal.
                                let id = self.node_builder.next_id();
                                // Set the type of the node ID.
                                self.type_table.insert(id, Type::Integer(IntegerType::U32));
                                id
                            },
                        ))),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, array.element_type().clone());
                            id
                        },
                    })));
                statements.push(stmt);

                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, array.element_type().clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                expression
            })
            .collect();

        // Construct the array expression.
        let (expr, stmts) = self.reconstruct_array(ArrayExpression {
            elements,
            span: Default::default(),
            id: {
                // Create a node ID for the array expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table.insert(id, Type::Array(array.clone()));
                id
            },
        });

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the array expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }

    pub(crate) fn ternary_struct(
        &mut self,
        struct_: &Composite,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each struct member, construct a new ternary expression.
        let members = struct_
            .members
            .iter()
            .map(|Member { identifier, type_, .. }| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Member(MemberAccess {
                        inner: Box::new(Expression::Identifier(*first)),
                        name: *identifier,
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Member(MemberAccess {
                        inner: Box::new(Expression::Identifier(*second)),
                        name: *identifier,
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, type_.clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                StructVariableInitializer {
                    identifier: *identifier,
                    expression: Some(expression),
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                }
            })
            .collect();

        let (expr, stmts) = self.reconstruct_struct_init(StructExpression {
            name: struct_.identifier,
            members,
            span: Default::default(),
            id: {
                // Create a new node ID for the struct expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table
                    .insert(id, Type::Composite(CompositeType { id: struct_.identifier, program: struct_.external }));
                id
            },
        });

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the struct expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }

    pub(crate) fn ternary_tuple(
        &mut self,
        tuple_type: &TupleType,
        condition: &Expression,
        first: &Identifier,
        second: &Identifier,
    ) -> (Expression, Vec<Statement>) {
        // Initialize a vector to accumulate any statements generated.
        let mut statements = Vec::new();
        // For each tuple element, construct a new ternary expression.
        let elements = tuple_type
            .elements()
            .iter()
            .enumerate()
            .map(|(i, type_)| {
                // Create an assignment statement for the first access expression.
                let (first, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(Expression::Identifier(*first)),
                        index: NonNegativeNumber::from(i),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);
                // Create an assignment statement for the second access expression.
                let (second, stmt) =
                    self.unique_simple_assign_statement(Expression::Access(AccessExpression::Tuple(TupleAccess {
                        tuple: Box::new(Expression::Identifier(*second)),
                        index: NonNegativeNumber::from(i),
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the access expression.
                            let id = self.node_builder.next_id();
                            // Set the type of the node ID.
                            self.type_table.insert(id, type_.clone());
                            id
                        },
                    })));
                statements.push(stmt);

                // Recursively reconstruct the ternary expression.
                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    // Access the member of the first expression.
                    if_true: Box::new(Expression::Identifier(first)),
                    // Access the member of the second expression.
                    if_false: Box::new(Expression::Identifier(second)),
                    span: Default::default(),
                    id: {
                        // Create a new node ID for the ternary expression.
                        let id = self.node_builder.next_id();
                        // Set the type of the node ID.
                        self.type_table.insert(id, type_.clone());
                        id
                    },
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                expression
            })
            .collect();

        // Construct the tuple expression.
        let tuple = TupleExpression {
            elements,
            span: Default::default(),
            id: {
                // Create a new node ID for the tuple expression.
                let id = self.node_builder.next_id();
                // Set the type of the node ID.
                self.type_table.insert(id, Type::Tuple(tuple_type.clone()));
                id
            },
        };
        let (expr, stmts) = self.reconstruct_tuple(tuple.clone());

        // Accumulate any statements generated.
        statements.extend(stmts);

        // Create a new assignment statement for the tuple expression.
        let (identifier, statement) = self.unique_simple_assign_statement(expr);

        statements.push(statement);

        (Expression::Identifier(identifier), statements)
    }
}
