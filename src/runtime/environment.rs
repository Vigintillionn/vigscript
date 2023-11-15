use std::collections::HashMap;

use crate::runtime::values;

#[derive(Debug)]
pub struct Environment {
  parent: Option<Box<Environment>>,
  variables: HashMap<String, values::RuntimeValue>
}

impl Environment {
  pub fn new(parent_env: Option<Environment>) -> Environment {
    let mut env = Environment {
      parent: None,
      variables: HashMap::new()
    };
    if let Some(parent) = parent_env {
      env.parent = Some(Box::new(parent));
    }
    env
  }

  pub fn declare_var(&mut self, name: String, value: values::RuntimeValue) -> values::RuntimeValue {
    if self.variables.contains_key(&name) {
      panic!("Variable {} already declared", name);
    }
    self.variables.insert(name, value.clone());
    value
  }

  pub fn assign_var(&mut self, name: String, value: &values::RuntimeValue) -> values::RuntimeValue {
    let env = self.resolve(name.clone());
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