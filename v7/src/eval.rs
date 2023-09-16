use crate::{
  values::Value,
  statements::Statement,
  context::Context,
  type_error, internal_error
};

#[allow(unused)]
pub fn eval_ast(ast: &[Statement], ctx: &mut Context) -> Value {
  for statement in ast {
    use Statement as S;
    match statement {
      S::Call { id, args } => {
        let value = ctx.stack.get_value(id).clone();
        if let Value::Function(func) = value {
          func.call(args, ctx);
        } else {
          type_error!("{id:?} is not a function!"; ctx);
        }
      }
      S::Declaration { id, mutable, expr } => {
        ctx.stack.declare(id.clone(), expr.solve());
      }
      S::Assignment { id, expr } => {
        ctx.stack.assign(id.clone(), expr.solve());
      },
      _ => internal_error!("not implemented")
    }
  }
  Value::Null
}