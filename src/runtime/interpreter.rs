use crate::runtime::values;
use crate::runtime::environment;
use core::panic;

use crate::parser::ast;

pub fn evaluate(prog: ast::Program, env: &mut environment::Environment) -> values::RuntimeValue {
  let mut last = values::RuntimeValue::Null;
  for stmt in prog.body {
    last = evaluate_node(stmt, env);
  }
  last
}

fn evaluate_node(node: ast::Stmt, env: &mut environment::Environment) -> values::RuntimeValue {
  match node {
    ast::Stmt::Expr(node) => evaluate_expr(node, env),
    ast::Stmt::VarDecl { muteable, name, value } => evaluate_var_decl(muteable, name, value, env),
    _ => panic!("Not implemented {:?}", node)
  }
}

fn evaluate_expr(node: ast::Expr, env: &mut environment::Environment) -> values::RuntimeValue {
  match node {
    ast::Expr::IntLit { value } => values::RuntimeValue::Number { value },
    ast::Expr::BinExp { left, op, right } => evaluate_binary_expr(*left, op, *right, env),
    ast::Expr::Ident { symbol } => evaluate_ident(symbol, env),
    _ => values::RuntimeValue::Null
  }
}

fn evaluate_binary_expr(left: ast::Expr, op: String, right: ast::Expr, env: &mut environment::Environment) -> values::RuntimeValue {
  let lhs = evaluate_expr(left, env);
  let rhs = evaluate_expr(right, env);

  match (lhs, rhs) {
    (values::RuntimeValue::Number { value: lhs }, values::RuntimeValue::Number { value: rhs }) => {
      match op.as_str() {
        "+" => values::RuntimeValue::Number { value: lhs + rhs },
        "-" => values::RuntimeValue::Number { value: lhs - rhs },
        "*" => values::RuntimeValue::Number { value: lhs * rhs },
        "/" => values::RuntimeValue::Number { value: lhs / rhs },
        "%" => values::RuntimeValue::Number { value: lhs % rhs },
        _ => values::RuntimeValue::Null
      }
    },
    _ => values::RuntimeValue::Null
  }
}

fn evaluate_ident(symbol: String, env: &mut environment::Environment) -> values::RuntimeValue {
  env.lookup_var(symbol)
}

fn evaluate_var_decl(muteable: bool, name: String, value: Option<ast::Expr>, env: &mut environment::Environment) -> values::RuntimeValue {
  let res = match value {
    Some(expr) => evaluate_expr(expr, env),
    None => values::RuntimeValue::Null
  };
  env.declare_var(name, res, muteable)
}