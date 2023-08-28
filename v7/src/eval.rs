use crate::{
  values::Value,
  aliases::Arguments,
  statments::Statment,
  context::Context,
  stack::Stack as _,
  internal_error,
};

#[allow(unused)]
pub fn eval_ast(ast: &Vec<Statment>, additional_data: Arguments, Context { stack, .. }: &mut Context) -> Value {
  for statment in ast {
    match statment {
      Statment::Call { id, args } => {
        stack.get_value(id);
      }
      // Assignment { id, expr } => {
      //   let value = resolve(expr);
      //   println!("set {} = {:?}", id, value);
      //   stack.set_value(id.clone(), *value);
      // },
      _ => internal_error!("not implemented")
    }
  }
  Value::None
}