
/// CodeGenContext tracks the state of the code generation process. 
/// This resolves type references and generate validation traits.
/// NOTE: maybe a type checker should be implemented between this and the AST generation.
pub struct CodeGenContext {
    types: HashMap<String, RustType>,
    validation_rules: HashMap<String, Vec<ValidationRule>>,
}

