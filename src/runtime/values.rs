use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
  Null,
  Number {
    value: f32
  },
  Bool {
    value: bool
  },
  Object(Object)
}

#[derive(Debug, Clone)]
pub struct Object {
  pub properties: HashMap<String, RuntimeValue>
}