use crate::types::Any;

pub enum FunctionAstRoot {
	Declaration(String, Any<'static>, Any<'static>)
}

pub struct Function {
	pub name: String,
	pub params: HashMap<String, (Type, Value)>, // sort
	pub rettype: Type,
	pub body: Vec<FunctionAstRoot>
}

pub fn parse_function(chars: &[char], i: &mut usize) -> Function {
	let mut name: String = String::new();
	Function {
		name,
		params,
		body
	}
}