//! Expression Evaluation
use evalexpr::{eval, EvalexprError, Value};
use regex::Regex;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Error Types for Expression Evaluator
#[derive(Clone, Debug)]
pub enum NamespaceError {
    VariableIdentifierNotFound(String),
    ExpressionEvaluationError(EvalexprError),
}

/// A namespace is a nested mapping of variable names to values
#[derive(Clone, Debug)]
pub enum NamespaceValue {
    Namespace(HashMap<String, NamespaceValue>),
    Value(String),
}

type Namespace = HashMap<String, NamespaceValue>;

/// A context to store variables and evaluate expressions
#[derive(Clone, Debug)]
pub struct ExpressionContext {
    namespace: NamespaceValue,
}

/// build a namespacevalue from a json value
fn json_to_namespace(json: &JsonValue) -> Option<NamespaceValue> {
    match json {
        JsonValue::Object(map) => {
            let mut namespace = Namespace::new();
            for (key, value) in map {
                let r = json_to_namespace(value)?;
                let _ = namespace.insert(key.clone(), r);
            }
            Some(NamespaceValue::Namespace(namespace))
        }
        JsonValue::String(s) => Some(NamespaceValue::Value(s.clone())),
        _ => None,
    }
}

/// fetch a variable from a namespace
fn fetch_variable(namespace: &NamespaceValue, path: &[&str]) -> Result<String, EvalexprError> {
    let (head, tail) = path.split_at(1);
    match namespace {
        NamespaceValue::Namespace(inner) if !tail.is_empty() => inner
            .get(head[0])
            .and_then(|next| fetch_variable(next, tail).ok())
            .ok_or_else(|| EvalexprError::VariableIdentifierNotFound(head[0].to_string())),
        NamespaceValue::Namespace(inner) if tail.is_empty() => match inner.get(head[0]) {
            Some(NamespaceValue::Value(value)) => Ok(value.clone()),
            _ => Err(EvalexprError::VariableIdentifierNotFound(
                head[0].to_string(),
            )),
        },
        NamespaceValue::Value(value) if tail.is_empty() => Ok(value.clone()),
        _ => Err(EvalexprError::VariableIdentifierNotFound(
            head[0].to_string(),
        )),
    }
}

impl ExpressionContext {
    /// create a namespace from a nested JSON dictionary
    pub fn from_json(json: &JsonValue) -> Option<ExpressionContext> {
        let namespace = json_to_namespace(json);
        match namespace {
            Some(namespace) => Some(ExpressionContext {
                namespace: namespace,
            }),
            None => None,
        }
    }

    /// get a variable from the namespace
    pub fn get(&self, path: &[&str]) -> Result<String, NamespaceError> {
        match fetch_variable(&self.namespace, path) {
            Ok(value) => Ok(value),
            Err(err) => Err(NamespaceError::ExpressionEvaluationError(err)),
        }
    }

    /// evaluate an expression in the namespace
    pub fn eval_expression(&self, expression: &str) -> Result<Value, NamespaceError> {
        let re = Regex::new(r"\$\{([A-Za-z0-9_/]+)\}").unwrap();

        // Check and replace all placeholders first
        let mut final_expression = String::from(expression);
        for caps in re.captures_iter(expression) {
            match self.get(&caps[1].split('/').collect::<Vec<_>>()) {
                Ok(value) => {
                    final_expression = final_expression.replace(&caps[0], &value);
                }
                Err(err) => return Err(err), // Return early if a variable could not be fetched
            }
        }

        // If all variables were successfully fetched, evaluate the expression
        match eval(&final_expression) {
            Ok(value) => Ok(value),
            Err(_) => Ok(final_expression.into()), // ugh, this is not so good
        }
    }
}
