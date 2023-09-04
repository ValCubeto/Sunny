use crate::{
  values::Value,
  statments::Statment,
  context::Context,
  stack::Stack as _, type_error, internal_error
};

#[allow(unused)]
pub fn eval_ast(ast: &Vec<Statment>, ctx: &mut Context) -> Value {
  for statment in ast {
    use Statment as S;
    match statment {
      S::Call { id, args } => {
        match ctx.stack.get_value(id) {
          Value::Function(func) => drop(func.call(args, ctx)),
          _ => type_error!("")
        }
      }
      S::Declaration { id, mutable, expr } => {
        todo!()
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