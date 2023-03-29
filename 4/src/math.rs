use std::collections::HashMap;

pub fn add(a: &String, b: &String) -> String {
	let mut result: String = String::new();
	result
}

pub fn sub(a: &String, b: &String) -> String {
	a.clone()
}

pub fn div(a: &String, b: &String) -> String {
	a.clone()
}

pub fn mul(a: &String, b: &String) -> String {
	a.clone()
}

pub fn rem(a: &String, b: &String) -> String {
	a.clone()
}

pub fn greater(n: &String, than: &String) -> bool {
	n.len() > than.len()
}

pub fn less(n: &String, than: &String) -> bool {
	n.len() < than.len()
}

pub fn greater_eq(n: &String, than: &String) -> bool {
	*n == *than || greater(n, than)
}

pub fn less_eq(n: &String, than: &String) -> bool {
	*n == *than || less(n, than)
}

pub fn pow(n: &String, power: &String) -> String {
	let mut result: String = n.clone();
	let mut i: String = String::from("0");
	let one: String = String::from("1");
	while less_eq(&i, power) {
		result = mul(&result, n);
		i = add(&i, &one);
	}
	result.clone()
}

pub fn sqrt(n: &String/* , base: &String */) -> String {
	let Ok(float) = n.parse::<f64>()
		else { return n.clone() };
	float.sqrt().to_string()
}

// crbt

struct Token;

struct Type(String, Vec<Token>);
struct Param(String, Option<Type>, Option<Value>);

enum Value {
	None,
	Bool(bool),
	Num(String),
	Str(String),
	List(Vec<Value>),
	Dict(HashMap<String, Value>),
	// name, super, entries
	Class(String, Option<String>, HashMap<Value, Value>),
	Instance(String, HashMap<Value, Value>),
	// name, params, rettype, body
	Fun(String, Vec<Param>, Type, Vec<Token>)
}

fn main() {
	let value: Value = Value::Dict(HashMap::<String, Value>::from([(String::from("age"), Value::Num(String::from("23")))]));
}