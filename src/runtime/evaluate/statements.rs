use crate::frontend::ast::{Expr, Stmt};
use crate::runtime::environment::Environment;
use crate::runtime::values::RuntimeValue;
use crate::runtime::evaluate::expressions::evaluate_expr;

pub fn evaluate_var_decl(mutable: bool, name: String, value: Option<Expr>, env: &mut Environment) -> RuntimeValue {
  let res = match value {
    Some(expr) => evaluate_expr(expr, env),
    None => RuntimeValue::Null
  };
  env.declare_var(name, res, mutable)
}

pub fn evaluate_func_decl(params: Vec<String>, name: String, body: Vec<Stmt>, env: &mut Environment) -> RuntimeValue {
  let func = RuntimeValue::Function {
    name: name.clone(),
    params: params.clone(),
    decl_env: env.clone(),
    body: body.clone()
  };
  env.declare_var(name, func, true)
}

pub fn evaluate_if_stmt(condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>>, env: &mut Environment) -> RuntimeValue {
  let condition = evaluate_expr(condition, env);
  match condition {
    RuntimeValue::Bool { value } => {
      if value == true {
        evaluate_branch(then_branch, env)
      } else {
        match else_branch {
          Some(branch) => evaluate_branch(branch, env),
          None => RuntimeValue::Null
        }
      }
    },
    RuntimeValue::Null => {
      match else_branch {
        Some(branch) => evaluate_branch(branch, env),
        None => RuntimeValue::Null
      }
    },
    _ => evaluate_branch(then_branch, env)
  }
}

pub fn evaluate_branch(branch: Vec<Stmt>, env: &mut Environment) -> RuntimeValue {
  let mut res: RuntimeValue = RuntimeValue::Null;
  for stmt in branch {
    match stmt {
      Stmt::Return { value } => {
        res = match value {
          Some(expr) => evaluate_expr(expr, env),
          None => RuntimeValue::Null
        };
        break;
      },
      Stmt::Expr(expr) => res = evaluate_expr(expr, env),
      Stmt::If { condition, then_branch, else_branch } => res = evaluate_if_stmt(condition, then_branch, else_branch, env),
      Stmt::For { ident, iterable, body } => res = evaluate_for_stmt(ident, *iterable, body, env),
      _ => res = RuntimeValue::Null
    }
  }
  res
}

pub fn evaluate_for_stmt(ident: String, iterable: Expr, body: Vec<Stmt>, env: &mut Environment) -> RuntimeValue {
  let iterable = evaluate_expr(iterable, env);
  match iterable {
    RuntimeValue::Array { elements } => {
      let mut res: RuntimeValue = RuntimeValue::Null;
      env.declare_var(ident.clone(), elements.first().unwrap().clone(), true);
      for element in elements {
        env.assign_var(ident.clone(), &element);
        res = evaluate_branch(body.clone(), env);
      }
      env.void_var(ident);
      res
    },
    _ => panic!("You can only iterate over arrays")
  }
}