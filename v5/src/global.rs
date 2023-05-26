use crate::types::{Dict, Type, Value};

pub fn init(exec_path: &String, flags: &[String], file_path: &String, args: &[String]) -> Dict {
	Dict::from([
		("process".to_string(), (Type::Defined(vec![]), Value::Dict(Dict::from([
			("exec_path".to_string(), (Type::Defined(vec![]), Value::String(exec_path.clone()))),
			("flags".to_string(), (Type::Defined(vec![]), Value::List(flags.iter().map(|v: &String| Value::String(v.clone())).collect()))),
			("file_path".to_string(), (Type::Defined(vec![]), Value::String(file_path.clone()))),
			("args".to_string(), (Type::Defined(vec![]), Value::List(args.iter().map(|v: &String| Value::String(v.clone())).collect()))),
			// ("get_title")
			// ("set_title", |title: String|)
		])))),
		("none".to_string(), (Type::Defined(vec![]), Value::None)),
		("infinity".to_string(), (Type::Defined(vec![]), Value::Infinity)),
	])
}