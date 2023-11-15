use crate::parser::ast::Expr;
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