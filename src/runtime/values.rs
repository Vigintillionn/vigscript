use std::collections::HashMap;

use crate::runtime::environment::Environment;
use crate::frontend::ast::Stmt;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
  Null,
  Number {
    value: f32
  },
  Bool {
    value: bool
  },
  Object(Object),
  NativeFunction {
    body: fn(Vec<RuntimeValue>, &mut Environment) -> RuntimeValue
  },
  Function {
    name: String,
    params: Vec<String>,
    decl_env: Environment,
    body: Vec<Stmt>
  },
  String {
    value: String
  }
}

#[derive(Debug, Clone)]
pub struct Object {
  pub properties: HashMap<String, RuntimeValue>
}