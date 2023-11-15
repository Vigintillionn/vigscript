use crate::runtime::values::RuntimeValue;
use crate::runtime::environment;
use crate::runtime::evaluate::expressions::evaluate_expr;
use crate::runtime::evaluate::statements::evaluate_var_decl;
use crate::parser::ast::{Program, Stmt};
use core::panic;


pub fn evaluate(prog: Program, env: &mut environment::Environment) -> RuntimeValue {
  let mut last = RuntimeValue::Null;
  for stmt in prog.body {
    last = evaluate_node(stmt, env);
  }
  last
}

fn evaluate_node(node: Stmt, env: &mut environment::Environment) -> RuntimeValue {
  match node {
    Stmt::Expr(node) => evaluate_expr(node, env),
    Stmt::VarDecl { muteable, name, value } => evaluate_var_decl(muteable, name, value, env),
    _ => panic!("Not implemented {:?}", node)
  }
}