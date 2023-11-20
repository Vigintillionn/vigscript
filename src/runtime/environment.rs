use std::collections::{HashMap, HashSet};
use crate::{runtime::values::{self, RuntimeValue}, frontend::ast::Expr};

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
          values::RuntimeValue::Array { elements } => {
            print!("[");
            for (i, element) in elements.iter().enumerate() {
              print!("{:?}", element);
              if i != elements.len() - 1 {
                print!(", ");
              }
            }
            print!("]");
          },
          _ => print!("{:?}", arg)
        }
      }
      print!("\n");
      values::RuntimeValue::Null
    }
  }, false);

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

  env.declare_var("Array".to_string(), values::RuntimeValue::Object(values:: Object {
    properties: {
      let mut map = HashMap::new();
      map.insert("new".to_string(), values::RuntimeValue::NativeFunction {
        body: |args, _| {
          let amount = match args.get(0) {
            Some(values::RuntimeValue::Number { value }) => *value as usize,
            _ => 0
          };
          let mut elements = Vec::new();
          for _ in 0..amount {
            elements.push(values::RuntimeValue::Null);
          }
          values::RuntimeValue::Array { elements }
        }
      });
      map.insert("from".to_string(), values::RuntimeValue::NativeFunction {
        body: |args, _| {
          let mut elements = Vec::new();
          for arg in args {
            elements.push(arg);
          }
          values::RuntimeValue::Array { elements }
        }
      });
      map.insert("has".to_string(), values::RuntimeValue::NativeFunction {
        body: |args, _| {
          let array = match args.get(0) {
            Some(values::RuntimeValue::Array { elements }) => elements,
            _ => panic!("First argument must be an array")
          };
          let value = match args.get(1) {
            Some(value) => value,
            _ => panic!("Second argument must be a value")
          };
          let mut has = false;
          for element in array {
            if element == value {
              has = true;
              break;
            }
          }
          values::RuntimeValue::Bool { value: has }
        }
      });
      map.insert("concat".to_string(), values::RuntimeValue::NativeFunction {
        body: |args, _| {
          let array = match args.get(0) {
            Some(values::RuntimeValue::Array { elements }) => elements,
            _ => panic!("First argument must be an array")
          };
          let value = match args.get(1) {
            Some(value) => value,
            _ => panic!("Second argument must be a value")
          };
          let mut elements = array.clone();
          elements.push(value.clone());
          values::RuntimeValue::Array { elements }
        }
      });
      map
    }
  }), false);

  env
}

#[derive(Debug, Clone, PartialEq)]
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

  pub fn void_var(&mut self, name: String) -> values::RuntimeValue {
    if self.variables.contains_key(&name) {
      self.variables.remove(&name.clone());
    }
    values::RuntimeValue::Null
  }

  pub fn assign_var(&mut self, name: String, value: &values::RuntimeValue) -> values::RuntimeValue {
    let env = self.resolve(name.clone());
    if env.constants.contains(&name) {
      panic!("Cannot assign to constant {}", name);
    }
    env.variables.insert(name, (*value).clone());
    value.clone()
  }

  pub fn lookup_object_member(&mut self, object: Option<Box<Expr>>, val: Option<RuntimeValue>, property: Box<Expr>, computed: bool) -> values::RuntimeValue {
    // values::RuntimeValue::Null
    /*
    Member { 
      object: Member { 
        object: Ident { symbol: "foo" }, 
        property: Ident { symbol: "bar" }, 
        computed: false 
      }, property: Ident { symbol: "complex" }, 
    computed: false }
    */
    println!("{:?}", object);
    println!("{:?}", property);
    match *object.clone().unwrap() {
      Expr::Ident { symbol } => {
        let env = self.resolve(symbol.clone());
        let past_val = env.lookup_var(symbol.clone());

        let obj = match past_val {
          values::RuntimeValue::Object(obj) => obj,
          _ => panic!("{} is not an object", symbol)
        };
        let prop = match *property {
          Expr::Ident { symbol } => symbol,
          _ => panic!("{:?} is not a valid property", property)
        };
        if obj.properties.contains_key(&prop) {
          obj.properties.get(&prop).unwrap().clone()
        } else {
          values::RuntimeValue::Null
        }
      },
      Expr::Member { object: o, property: p, computed: c } => self.lookup_object_member(
        Some(o),
        None, 
        p, 
        c
      ),
      _ => panic!("Not implemented")
    }

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