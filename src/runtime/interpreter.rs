use crate::runtime::values::RuntimeValue;
use crate::runtime::environment;
use crate::runtime::evaluate::expressions::evaluate_expr;
use crate::runtime::evaluate::statements::{evaluate_var_decl, evaluate_func_decl, evaluate_if_stmt, evaluate_for_stmt};
use crate::frontend::ast::{Program, Stmt};


pub fn evaluate(prog: Program, env: &mut environment::Environment) -> RuntimeValue {
  let mut last = RuntimeValue::Null;
  for stmt in prog.body {
    last = evaluate_node(stmt, env);
  }
  last
}

pub fn evaluate_node(node: Stmt, env: &mut environment::Environment) -> RuntimeValue {
  match node {
    Stmt::Expr(node) => evaluate_expr(node, env),
    Stmt::VarDecl { mutable, name, value } => evaluate_var_decl(mutable, name, value, env),
    Stmt::FuncDecl { params, name, body } => evaluate_func_decl(params, name, body, env),
    Stmt::Return { value: _ } => panic!("You can only return from inside a function."),
    Stmt::If { condition, then_branch, else_branch } => evaluate_if_stmt(condition, then_branch, else_branch, env),
    Stmt::For { ident, iterable, body } => evaluate_for_stmt(ident, *iterable, body, env),
    _ => panic!("Not implemented {:?}", node)
  }
}