use std::collections::HashMap;

use crate::runtime::environment::Environment;
use crate::runtime::values::{RuntimeValue, Object};
use crate::parser::ast::{Expr, Property, Stmt};

pub fn evaluate_expr(node: Expr, env: &mut Environment) -> RuntimeValue {
  match node {
    Expr::IntLit { value } => RuntimeValue::Number { value },
    Expr::BinExp { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
    Expr::Ident { symbol } => evaluate_ident(symbol, env),
    Expr::Assign { assignee, value } => evaluate_assignment(*assignee, *value, env),
    Expr::ObjectLit { properties } => evaluate_object_expr(properties, env),
    Expr::Call { callee, args } => evaluate_call_expr(callee, args, env),
    Expr::Member { object: _, property: _, computed: _ } => evaluate_member_expr(node, env),
    Expr::String { value } => RuntimeValue::String { value },
    _ => panic!("Not implemented {:?}", node)//values::RuntimeValue::Null
  }
}

pub fn evaluate_binary_expr(left: Expr, op: String, right: Expr, env: &mut Environment) -> RuntimeValue {
  let lhs = evaluate_expr(left, env);
  let rhs = evaluate_expr(right, env);

  match (lhs, rhs) {
    (RuntimeValue::Number { value: lhs }, RuntimeValue::Number { value: rhs }) => {
      match op.as_str() {
        "+" => RuntimeValue::Number { value: lhs + rhs },
        "-" => RuntimeValue::Number { value: lhs - rhs },
        "*" => RuntimeValue::Number { value: lhs * rhs },
        "/" => RuntimeValue::Number { value: lhs / rhs },
        "%" => RuntimeValue::Number { value: lhs % rhs },
        "==" => RuntimeValue::Bool { value: equals(RuntimeValue::Number { value: lhs }, RuntimeValue::Number { value: rhs }, true) },
        "!=" => RuntimeValue::Bool { value: equals(RuntimeValue::Number { value: lhs }, RuntimeValue::Number { value: rhs }, false) },
        "<" => RuntimeValue::Bool { value: lhs < rhs },
        ">" => RuntimeValue::Bool { value: lhs > rhs },
        _ => RuntimeValue::Null
      }
    },
    (RuntimeValue::String { value: lhs }, RuntimeValue::String { value: rhs }) => {
      match op.as_str() {
        "+" => RuntimeValue::String { value: lhs + &rhs },
        "==" => RuntimeValue::Bool { value: equals(RuntimeValue::String { value: lhs }, RuntimeValue::String { value: rhs }, true) },
        "!=" => RuntimeValue::Bool { value: equals(RuntimeValue::String { value: lhs }, RuntimeValue::String { value: rhs }, false) },
        _ => RuntimeValue::Null
      }
    },
    (RuntimeValue::Bool { value: lhs }, RuntimeValue::Bool { value: rhs }) => {
      match op.as_str() {
        "==" => RuntimeValue::Bool { value: equals(RuntimeValue::Bool { value: lhs }, RuntimeValue::Bool { value: rhs }, true) },
        "!=" => RuntimeValue::Bool { value: equals(RuntimeValue::Bool { value: lhs }, RuntimeValue::Bool { value: rhs }, false) },
        _ => RuntimeValue::Null
      }
    },
    _ => RuntimeValue::Null
  }
}

pub fn evaluate_ident(symbol: String, env: &mut Environment) -> RuntimeValue {
  env.lookup_var(symbol)
}

pub fn evaluate_object_expr(properties: Vec<Property>, env: &mut Environment) -> RuntimeValue {
  let mut object = Object { properties: HashMap::new() };

  for prop in properties {
    let runtime_val = match prop.value {
      None => env.lookup_var(prop.key.clone()),
      Some(expr) => evaluate_expr(*expr, env)
    };
    object.properties.insert(prop.key, runtime_val);
  }

  RuntimeValue::Object(object)
}

pub fn evaluate_assignment(assignee: Expr, value: Expr, env: &mut Environment) -> RuntimeValue {
  match assignee {
    Expr::Member { object: _, property: _, computed: _ } => evaluate_member_expr(assignee, env),
    Expr::Ident { symbol } => {
      let res = evaluate_expr(value, env);
      env.assign_var(symbol, &res)
    },
    _ => panic!("You can't assign to {:?}", assignee)
  }
}

pub fn evaluate_call_expr(callee: Box<Expr>, args: Vec<Expr>, env: &mut Environment) -> RuntimeValue {
  let runtime_args: Vec<RuntimeValue> = args.iter().map(|arg| evaluate_expr(arg.clone(), env)).collect();
  let func = evaluate_expr(*callee, env);

  match func {
    RuntimeValue::NativeFunction { body } => body(runtime_args, env),
    RuntimeValue::Function { name: _, params, decl_env, body } => {
      let mut scope = Environment::new(Some(decl_env));
      if params.len() != runtime_args.len() {
        panic!("Wrong number of arguments passed to function");
      }
      for (i, arg) in runtime_args.iter().enumerate() {
        scope.declare_var(params[i].clone(), arg.clone(), true);
      }

      let mut res: RuntimeValue = RuntimeValue::Null;
      for stmt in body {
        
        match stmt {
          Stmt::Return { value } => {
            res = match value {
              Some(expr) => evaluate_expr(expr, &mut scope),
              None => RuntimeValue::Null
            };
            break;
          },
          Stmt::Expr(expr) => res = evaluate_expr(expr, &mut scope),
          _ => res = RuntimeValue::Null
        }
      }
      res
    },
    _ => panic!("You can only call functions")
  }
}

pub fn evaluate_member_expr(node: Expr, env: &mut Environment) -> RuntimeValue {
  // env.lookup_object_member(node)
  match node {
    Expr::Member { object, property, computed } => env.lookup_object_member(object, property, computed, None),
    Expr::Ident { symbol } => env.lookup_var(symbol),
    _ => panic!("Oh oh")
  }
}

pub fn equals(lhs: RuntimeValue, rhs: RuntimeValue, value_if_eq: bool) -> bool {
  let equals = match (lhs, rhs) {
    (RuntimeValue::Number { value: lhs }, RuntimeValue::Number { value: rhs }) => lhs == rhs,
    (RuntimeValue::String { value: lhs }, RuntimeValue::String { value: rhs }) => lhs == rhs,
    (RuntimeValue::Bool { value: lhs }, RuntimeValue::Bool { value: rhs }) => lhs == rhs,
    (RuntimeValue::Null, RuntimeValue::Null) => true,
    _ => false
  };
  if equals {
    return value_if_eq;
  } else {
    return !value_if_eq;
  }
}