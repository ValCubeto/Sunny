use crate::types::Any;

enum FunctionAst {
	Declaration(String, Any<'static>, Any<'static>)
}

struct Function {
	name: String,
	body: Vec<FunctionAst>
}

pub fn parse_function(chars: &[char], i: &mut usize) -> Function {
	let mut name: String = String::new();
	Function {
		name,
		body: vec![
			FunctionAst::Declaration("println".to_string(), Any::String("a".to_string()), Any::String("a".to_string()))
		]
	}
}