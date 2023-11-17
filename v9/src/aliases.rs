use std::{ rc::Rc, collections::HashMap };
use crate::values::Value;

pub type Id = Rc<str>;
pub type ArrayPtr = Rc<Vec<Value>>;
pub type DictPtr = Rc<HashMap<Id, Value>>;

// Fn(Arguments) -> Value
// type Arguments = Vec<Value>;
// enum Action { Exit(ExitCode), }
// std::process::exit(code: ExitCode) { this.exit_code = Some(code); }
// prevent crashing when using Function::try()
pub type Arguments = Vec<Value>;
pub type FunctionPtr = Rc<dyn Fn(Arguments) -> Value>;
