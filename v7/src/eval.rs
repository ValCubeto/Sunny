use crate::{
  values::Value,
  statments::Statment,
  context::Context,
  stack::Stack as _,
  internal_error,
};

#[allow(unused)]
pub fn eval_ast(ast: &Vec<Statment>, ctx: &mut Context) -> Value {
  for statment in ast {
    match statment {
      Statment::Call { id, args } => {
        ctx.stack.get_value(id);
      }
      // Assignment { id, expr } => {
      //   let value = resolve(expr);
      //   println!("set {} = {:?}", id, value);
      //   stack.set_value(id.clone(), *value);
      // },
      _ => internal_error!("not implemented")
    }
  }
  Value::Null
}