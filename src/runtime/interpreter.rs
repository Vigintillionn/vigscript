use crate::runtime::values::RuntimeValue;
use crate::runtime::environment;
use crate::runtime::evaluate::expressions::evaluate_expr;
use crate::runtime::evaluate::statements::{evaluate_var_decl, evaluate_func_decl};
use crate::parser::ast::{Program, Stmt};
use core::panic;


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
    _ => panic!("Not implemented {:?}", node)
  }
}