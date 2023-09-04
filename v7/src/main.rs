use crate::{
  argv::{ ParsedArgs, parse_args },
  files::read_file,
  aliases::{ Id, Arguments },
  context::Context,
  namespaces::parse_namespace,
  values::Value,
  globals::make_global,
  stack::Stack as _,
};

/// TODO:
/// stack: Rc<Dict> = [global, ...uppers, current, recent (maybe)]
/// check arguments' types, quantity, etc

fn main() {
  let mut args: ParsedArgs = parse_args();	
  let data = read_file(&mut args.main_path);

  println!("args = {args:#?}");
  println!("data = {data:?}");
  println!();

  let file_id = Id::from(args.main_path
    .file_name()
    .unwrap() // guaranteed to be a file
    .to_string_lossy()
    .to_string());
  let path_id = Id::from(args.main_path
    .to_string_lossy()
    .to_string());

  let mut ctx = Context::new(path_id, &data);
  ctx.stack.preppend(make_global());
  let main = parse_namespace(&mut ctx, file_id);
  
  let entrypoint = match main.public.get(&Id::from("main")).cloned() {
    None => reference_error!("main function not found"; ctx),
    Some(value) => value
  };
  ctx.stack.preppend(main.public);
  dbg!(&ctx.stack);

  if let Value::Function(function) = entrypoint {
    let arguments = Arguments::new();
    function.call(arguments, &mut ctx);
  } else {
    type_error!("missing main function"; ctx);
  }
}

mod about;
mod macros;
mod argv;
mod colors;
mod aliases;
mod files;
mod context;
mod namespaces;
mod values;
mod functions;
mod statments;
mod numbers;
mod expressions;
mod eval;
mod stack;
mod structs;
mod instances;
mod globals;

#[cfg(test)]
mod tests;