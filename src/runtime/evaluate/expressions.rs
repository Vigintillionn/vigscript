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