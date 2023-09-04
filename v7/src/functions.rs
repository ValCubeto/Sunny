use crate::{
  // numbers::collect_num,
  context::Context,
  aliases::{ Id, Arguments },
  statments::Statment,
  expressions::parse_expr,
  values::Value,
  syntax_error
};

pub fn parse_function(ctx: &mut Context, name: Id) -> Function {
  let mut body: Vec<Statment> = Vec::new();

  ctx.go();

  if ctx.current == '<' {
    ctx.next_char();
    ctx.go();
    while ctx.current != '>' {
      ctx.next_char();
      syntax_error!("function generics not implemented"; ctx);
    }
  }

  ctx.go();

  if ctx.current != '(' {
    syntax_error!("expected '(', found {:?}", ctx.current; ctx);
  }
  ctx.next_char();
  ctx.go();
  while ctx.current != ')' {
    ctx.next_char();
    syntax_error!("function parameters not implemented"; ctx);
  }
  ctx.next_char();
  ctx.go();

  if ctx.current == '-' {
    ctx.next_char();
    if ctx.current != '>' {
      syntax_error!("expected '>' (to complete '->'), found {:?}", ctx.current; ctx);
    }
    ctx.next_char();
    ctx.go();
    syntax_error!("functions' return type not implemented"; ctx);
  }

  if ctx.current != '{' {
    syntax_error!("expected '{{' or '->', found {:?}", ctx.current; ctx);
  }
  ctx.next_char();
  ctx.go();

  while ctx.current != '}' {
    if ctx.current.is_ascii_digit() {
      syntax_error!("numbers not implemented"; ctx);
      // let number = collect_num(ctx);
      // dbg!(&number);
      // break 'sub
    }
    if ctx.current.is_alphabetic() {
      let word = ctx.collect_word();
      let word = word.as_str();
      ctx.go();
      match word {
        "const" | "var" => {
          let is_const = word == "const";
          if ctx.current == '{' || ctx.current == '[' {
            syntax_error!("destructuring not implemented"; ctx);
          } else if ctx.current.is_alphabetic() {
            let id = Id::from(ctx.collect_word());
            ctx.go();
            if ctx.current == ':' {
              syntax_error!("typed {}s not implemented", if is_const { "constant" } else { "variable" }; ctx);
            }
            if ctx.current != '=' {
              syntax_error!("expected '=', got {:?}", ctx.current; ctx);
            }
            ctx.next_char();
            ctx.go();
            body.push(Statment::Declaration { id, mutable: !is_const, expr: parse_expr(ctx) });
          } else {
            syntax_error!("unexpected character {:?}", ctx.current; ctx);
          }
        },
        _ => {
          match ctx.current {
            '=' => {
              ctx.next_char();
              let expr = parse_expr(ctx);
              body.push(Statment::Assignment {
                id: Id::from(word),
                expr
              })
            }
            '(' => {
              ctx.next_char();
              while ctx.current != ')' {
                #[allow(unused)]
                let expr = parse_expr(ctx);
                ctx.next_char();
                syntax_error!("function call params not implemented"; ctx);
                // ctx.go();
              }
              ctx.next_char();
            }
            _ => syntax_error!("unexpected character {:?}", ctx.current; ctx)
          }
          syntax_error!("unexpected character {:?}", ctx.current; ctx);
        }
        }
      };
    ctx.next_char();
    ctx.go();
  }

  Function { name, value: FunctionValue::Defined(body) }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
  pub name: Id,
  pub value: FunctionValue
}

impl Function {
  pub fn call(&self, args: Arguments) -> Result<Value, FunError> {
    use FunctionValue as F;
    match self.value {
      F::Builtin(func) => func(args),
      F::Defined(ref func) => {
        func;
        Ok(Value::Null)
      }
    }
  }
}

#[allow(unused)]
#[derive(Debug)]
pub struct FunError {
  name: Id,
  description: Id
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum FunctionValue {
  // Vec<Statment>
  // Value::Instance(Instance { parent: (Rc<Struct>) name, values: [(Id) desc] })
  Builtin(fn(Arguments) -> Result<Value, FunError>),
  Defined(Vec<Statment>)
}
