//! Example of how the expession evaluation engine could work using the evalexpr crate
use evalexpr::*;
use regex::Regex;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum NamespaceValue {
    Namespace(HashMap<String, NamespaceValue>),
    Value(String),
}

type Namespace = HashMap<String, NamespaceValue>;

fn json_to_namespace(json: &JsonValue) -> Option<NamespaceValue> {
    match json {
        JsonValue::Object(map) => {
            let mut namespace = Namespace::new();
            for (key, value) in map {
                namespace.insert(key.clone(), json_to_namespace(value)?);
            }
            Some(NamespaceValue::Namespace(namespace))
        }
        JsonValue::String(s) => Some(NamespaceValue::Value(s.clone())),
        _ => None,
    }
}

fn main() {
    let json_string = r#"
        {
            "CFE_MISSION": {
                "MAX_CPU_ADDRESS_SIZE": "32"
            },
            "POWER": "2"
        }
    "#;

    let json: JsonValue = serde_json::from_str(json_string).unwrap();
    let namespaces = json_to_namespace(&json).unwrap();
    println!("{:#?}", namespaces);

    let expression = "${CFE_MISSION/MAX_CPU_ADDRESS_SIZE}/test.cfg";
    match eval_namespaced_expression(expression, &namespaces) {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("Error evaluating expression: {:?}", err),
    }
}

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

fn eval_namespaced_expression(
    expression: &str,
    namespaces: &NamespaceValue,
) -> Result<evalexpr::Value, EvalexprError> {
    let re = Regex::new(r"\$\{([A-Za-z0-9_/]+)\}").unwrap();

    // Check and replace all placeholders first
    let mut final_expression = String::from(expression);
    for caps in re.captures_iter(expression) {
        match fetch_variable(namespaces, &caps[1].split('/').collect::<Vec<_>>()) {
            Ok(value) => {
                final_expression = final_expression.replace(&caps[0], &value);
            }
            Err(err) => return Err(err), // Return early if a variable could not be fetched
        }
    }

    // If all variables were successfully fetched, evaluate the expression
    eval(&final_expression)
}
