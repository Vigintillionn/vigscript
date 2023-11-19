use std::collections::{HashMap, HashSet};
use crate::{runtime::values::{self, RuntimeValue}, parser::ast::Expr};

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

  pub fn lookup_object_member(&mut self, object: Box<Expr>, property: Box<Expr>, computed: bool, val: Option<RuntimeValue>) -> values::RuntimeValue {
    values::RuntimeValue::Null
    /*
    Member { 
      object: Member { 
        object: Ident { symbol: "foo" }, 
        property: Ident { symbol: "bar" }, 
        computed: false 
      }, property: Ident { symbol: "complex" }, 
    computed: false }
     */
  }

    // let prop = match *property {
    //   Expr::Ident { symbol } => symbol,
    //   _ => panic!("Not implemented")
    // };

    // if val.is_some() {
    //   let obj = match val.unwrap() {
    //     values::RuntimeValue::Object(obj) => obj,
    //     _ => panic!("Not implemented")
    //   };
    //   let res = obj.properties.get(&prop).unwrap().clone();
    //   println!("res {:?}", res);
    //   return res;
    // }

    // match *object {
    //   Expr::Member { object, property, computed: _ } => {
    //     let parent = object.clone();
    //     println!("parent {:?}", parent);
    //     let symbol = match *parent {
    //       Expr::Ident { symbol } => symbol,
    //       Expr::Member { object, property, computed } => {
    //         let obj = match *object.clone() {
    //           Expr::Ident { symbol } => symbol,
    //           _ => panic!("Not implemented")
    //         };
    //         let prop = match *property.clone() {
    //           Expr::Ident { symbol } => symbol,
    //           _ => panic!("Not implemented")
    //         };
    //         let res = self.lookup_object_member(object, property, computed, None);
    //         return res
    //       }
    //       _ => panic!("Not implemented")
    //     };
    //     let env = self.resolve(symbol.clone());
    //     let past_val = env.lookup_var(symbol.clone()); 

    //     let property_ident = Expr::Ident { symbol: prop.clone() };
    //     let obj = match past_val {
    //       values::RuntimeValue::Object(obj) => obj,
    //       _ => panic!("Not implemented")
    //     };
    //     let member = match *property {
    //       Expr::Ident { symbol } => symbol,
    //       _ => panic!("Not implemented")
    //     };
    //     let res = obj.properties.get(&member).unwrap().clone();
    //     println!("res {:?}", res);

    //     self.lookup_object_member(object, Box::new(property_ident), computed, Some(res))
    //   },
    //   Expr::Ident { symbol } => {
    //     let env = self.resolve(symbol.clone());
    //     let past_val = env.lookup_var(symbol.clone());

    //     let obj = match past_val {
    //       values::RuntimeValue::Object(obj) => obj,
    //       _ => panic!("Not implemented")
    //     };

    //     if obj.properties.contains_key(&prop) {
    //       obj.properties.get(&prop).unwrap().clone()
    //     } else {
    //       values::RuntimeValue::Null
    //     }
    //   },
    //   _ => panic!("Not implemented {:?}", object)
    // }

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