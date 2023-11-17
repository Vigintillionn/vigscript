use std::collections::{HashMap, HashSet};
use crate::runtime::values;

pub fn create_global_environment() -> Environment {
  let mut env = Environment::new(None);
  env.declare_var("true".to_string(), values::RuntimeValue::Bool { value: true  }, false);
  env.declare_var("false".to_string(), values::RuntimeValue::Bool { value: false }, false);
  env.declare_var("null".to_string(), values::RuntimeValue::Null, false);

  env.declare_var("print!".to_string(), values::RuntimeValue::NativeFunction { 
    body: |args, _| {
      for arg in args {
        match arg {
          values::RuntimeValue::String { value } => print!("{}", value),
          values::RuntimeValue::Number { value } => print!("{}", value),
          values::RuntimeValue::Bool { value } => print!("{}", value),
          values::RuntimeValue::Null => print!("null"),
          _ => print!("{:?}", arg)
        }
      }
      values::RuntimeValue::Null
    }
  }, false);
  // env.declare_var("now".to_string(), values::RuntimeValue::NativeFunction {
  //   body: |_, _| {
  //     let start = std::time::SystemTime::now();
  //     let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
  //     values::RuntimeValue::Number { value: since_the_epoch.as_secs_f32() }
  //   }
  // }, false);

  env.declare_var("Date".to_string(), values::RuntimeValue::Object(values::Object {
    properties: {
      let mut map = HashMap::new();
      map.insert("now".to_string(), values::RuntimeValue::NativeFunction {
        body: |_, _| {
          let start = std::time::SystemTime::now();
          let since_the_epoch = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
          values::RuntimeValue::Number { value: since_the_epoch.as_secs_f32() }
        }
      });
      map
    }
  }), false);

  env
}

#[derive(Debug, Clone)]
pub struct Environment {
  parent: Option<Box<Environment>>,
  variables: HashMap<String, values::RuntimeValue>,
  constants: HashSet<String>
}

impl Environment {
  pub fn new(parent_env: Option<Environment>) -> Environment {
    let mut env = Environment {
      parent: None,
      variables: HashMap::new(),
      constants: HashSet::new()
    };
    if let Some(parent) = parent_env {
      env.parent = Some(Box::new(parent));
    }
    env
  }

  pub fn declare_var(&mut self, name: String, value: values::RuntimeValue, mutable: bool) -> values::RuntimeValue {
    if self.variables.contains_key(&name) {
      panic!("Variable {} already declared", name);
    }
    if !mutable  {
      self.constants.insert(name.clone());
    }
    self.variables.insert(name, value.clone());
    value
  }

  pub fn assign_var(&mut self, name: String, value: &values::RuntimeValue) -> values::RuntimeValue {
    let env = self.resolve(name.clone());
    if env.constants.contains(&name) {
      panic!("Cannot assign to constant {}", name);
    }
    env.variables.insert(name, (*value).clone());
    value.clone()
  }

  pub fn lookup_var(&mut self, name: String) -> values::RuntimeValue {
    let env = self.resolve(name.clone());
    match env.variables.get(&name) {
      Some(value) => (*value).clone(),
      None => panic!("Variable {} is not defined", name)
    }
  }

  pub fn resolve(&mut self, varname: String) -> &mut Environment {
    if self.variables.contains_key(&varname) {
      return self
    }
    match self.parent {
      Some(ref mut parent) => parent.resolve(varname),
      None => panic!("Variable {} is not defined", varname)
    }
  }
}