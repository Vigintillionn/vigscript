mod values;
use core::panic;

use crate::parser::ast;

pub fn evaluate(prog: ast::Program) -> values::RuntimeValue {
  let mut last = values::RuntimeValue::Null;
  for stmt in prog.body {
    last = evaluate_node(stmt);
  }
  last
}

fn evaluate_node(node: ast::Stmt) -> values::RuntimeValue {
  match node {
    ast::Stmt::Expr(node) => evaluate_expr(node),
    _ => panic!("Not implemented")
  }
}

fn evaluate_expr(node: ast::Expr) -> values::RuntimeValue {
  match node {
    ast::Expr::IntLit { value } => values::RuntimeValue::Number { value },
    ast::Expr::BinExp { left, op, right } => evaluate_binary_expr(*left, op, *right),
    _ => values::RuntimeValue::Null
  }
}

fn evaluate_binary_expr(left: ast::Expr, op: String, right: ast::Expr) -> values::RuntimeValue {
  let lhs = evaluate_expr(left);
  let rhs = evaluate_expr(right);

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