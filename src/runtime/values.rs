use std::collections::HashMap;

use crate::runtime::environment::Environment;
use crate::frontend::ast::Stmt;

#[derive(Debug, Clone, PartialEq)]
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
  },
  Array {
    elements: Vec<RuntimeValue>
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
  pub properties: HashMap<String, RuntimeValue>
}