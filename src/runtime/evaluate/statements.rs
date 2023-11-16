use crate::parser::ast::{Expr, Stmt};
use crate::runtime::environment::Environment;
use crate::runtime::values::RuntimeValue;
use crate::runtime::evaluate::expressions::evaluate_expr;

pub fn evaluate_var_decl(muteable: bool, name: String, value: Option<Expr>, env: &mut Environment) -> RuntimeValue {
  let res = match value {
    Some(expr) => evaluate_expr(expr, env),
    None => RuntimeValue::Null
  };
  env.declare_var(name, res, muteable)
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