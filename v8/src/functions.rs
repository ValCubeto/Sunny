// dependencies: Vec<Rc<Value>>

struct Function {
	name: Id,
	params: Arguments,
	value: FunctionValue
}

enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, Id>),
	Defined(Vec<Statment>)
}

enum Statment {
	If {
		condition: Expression,
		body: Vec<Statment>,
		else_body: Vec<Statment>
	},
	Assignment {
		id: Vec<Id>,
		value: Expression
	},
	Call {
		id: Vec<Id>,
		args: Arguments
	}
}