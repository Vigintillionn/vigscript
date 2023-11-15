use std::collections::{HashMap, HashSet};

use crate::runtime::values;

#[derive(Debug)]
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

  pub fn declare_var(&mut self, name: String, value: values::RuntimeValue, muteable: bool) -> values::RuntimeValue {
    if self.variables.contains_key(&name) {
      panic!("Variable {} already declared", name);
    }
    if !muteable  {
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