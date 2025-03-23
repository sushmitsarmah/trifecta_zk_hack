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

use crate::{ParserContext, SpannedToken, tokenizer};

use leo_ast::{NodeBuilder, NodeID, Statement};
use leo_errors::{LeoError, emitter::Handler};
use leo_span::{
    Span,
    source_map::FileName,
    symbol::{SessionGlobals, create_session_if_not_set_then},
};
use leo_test_framework::{
    Test,
    runner::{Namespace, ParseType, Runner},
};
use serde::Serialize;
use tokenizer::Token;
use toml::Value;

type CurrentNetwork = snarkvm::prelude::MainnetV0;

// TODO: Enable parser warnings for passing tests

struct TokenNamespace;

impl Namespace for TokenNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Line
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        create_session_if_not_set_then(|s| {
            tokenize(test, s).map(|tokens| {
                Value::String(tokens.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
            })
        })
    }
}

fn not_fully_consumed(tokens: &mut ParserContext<CurrentNetwork>) -> Result<(), String> {
    if !tokens.has_next() {
        return Ok(());
    }
    let mut out = "did not consume all input: ".to_string();
    while tokens.has_next() {
        tokens.bump();
        out.push_str(&tokens.prev_token.to_string());
        out.push('\n');
    }
    Err(out)
}

fn with_handler<T>(
    tokens: Vec<SpannedToken>,
    logic: impl FnOnce(&mut ParserContext<'_, CurrentNetwork>) -> Result<T, LeoError>,
) -> Result<T, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = NodeBuilder::default();
    let mut tokens = ParserContext::new(&handler, &node_builder, tokens);
    let parsed = handler
        .extend_if_error(logic(&mut tokens))
        .map_err(|_| buf.extract_errs().to_string() + &buf.extract_warnings().to_string())?;
    not_fully_consumed(&mut tokens)?;
    Ok(parsed)
}

fn tokenize(test: Test, s: &SessionGlobals) -> Result<Vec<SpannedToken>, String> {
    let sf = s.source_map.new_source(&test.content, FileName::Custom("test".into()));
    tokenizer::tokenize(&sf.src, sf.start_pos).map_err(|x| x.to_string())
}

fn all_are_comments(tokens: &[SpannedToken]) -> bool {
    tokens.iter().all(|x| matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
}

fn toml_or_fail<T: Serialize>(value: T) -> Value {
    Value::try_from(value).expect("serialization failed")
}

struct ParseExpressionNamespace;

impl Namespace for ParseExpressionNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Line
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        create_session_if_not_set_then(|s| {
            let tokenizer = tokenize(test, s)?;
            if all_are_comments(&tokenizer) {
                return Ok(toml_or_fail(""));
            }
            with_handler(tokenizer, |p| p.parse_expression()).map(toml_or_fail)
        })
    }
}

struct ParseStatementNamespace;

impl Namespace for ParseStatementNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::ContinuousLines
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        create_session_if_not_set_then(|s| {
            let tokenizer = tokenize(test, s)?;
            if all_are_comments(&tokenizer) {
                return Ok(toml_or_fail(Statement::dummy(Span::default(), NodeID::default())));
            }
            with_handler(tokenizer, |p| p.parse_statement()).map(toml_or_fail)
        })
    }
}

struct ParseNamespace;

impl Namespace for ParseNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        create_session_if_not_set_then(|s| with_handler(tokenize(test, s)?, |p| p.parse_program()).map(toml_or_fail))
    }
}

struct SerializeNamespace;

// Helper functions to recursively filter keys from AST JSON.
// Redeclaring here since we don't want to make this public.
fn remove_key_from_json(value: &mut serde_json::Value, key: &str) {
    match value {
        serde_json::value::Value::Object(map) => {
            map.remove(key);
            for val in map.values_mut() {
                remove_key_from_json(val, key);
            }
        }
        serde_json::value::Value::Array(values) => {
            for val in values.iter_mut() {
                remove_key_from_json(val, key);
            }
        }
        _ => (),
    }
}

// Helper function to normalize AST
// Redeclaring here because we don't want to make this public
fn normalize_json_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(vec) => {
            let orig_length = vec.len();
            let mut new_vec: Vec<serde_json::Value> = vec
                .into_iter()
                .filter(|v| !matches!(v, serde_json::Value::Object(map) if map.is_empty()))
                .map(normalize_json_value)
                .collect();

            if orig_length == 2 && new_vec.len() == 1 {
                new_vec.pop().unwrap()
            } else {
                serde_json::Value::Array(new_vec)
            }
        }
        serde_json::Value::Object(map) => {
            serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, normalize_json_value(v))).collect())
        }
        _ => value,
    }
}

impl Namespace for SerializeNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        create_session_if_not_set_then(|s| {
            let tokenizer = tokenize(test, s)?;
            let parsed = with_handler(tokenizer, |p| p.parse_program())?;

            let mut json = serde_json::to_value(parsed).expect("failed to convert to json value");
            remove_key_from_json(&mut json, "span");
            json = normalize_json_value(json);

            Ok(serde_json::from_value::<toml::Value>(json).expect("failed serialization"))
        })
    }
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Parse" => Box::new(ParseNamespace),
            "ParseExpression" => Box::new(ParseExpressionNamespace),
            "ParseStatement" => Box::new(ParseStatementNamespace),
            "Serialize" => Box::new(SerializeNamespace),
            "Token" => Box::new(TokenNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn parser_tests() {
    leo_test_framework::run_tests(&TestRunner, "parser");
}
