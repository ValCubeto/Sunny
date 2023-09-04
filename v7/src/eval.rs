use crate::{
  values::Value,
  statments::Statment,
  context::Context,
  stack::Stack as _, type_error, internal_error
};

#[allow(unused)]
pub fn eval_ast(ast: &[Statment], ctx: &mut Context) -> Value {
  for statment in ast {
    use Statment as S;
    match statment {
      S::Call { id, args } => {
        let value = ctx.stack.get_value(id).clone();
        if let Value::Function(func) = value {
          func.call(args, ctx);
        } else {
          type_error!("{id:?} is not a function!"; ctx);
        }
      }
      S::Declaration { id, mutable, expr } => {
        todo!("declaration")
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