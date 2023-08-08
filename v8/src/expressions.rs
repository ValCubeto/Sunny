pub enum Expression {
	Value(Value),

	Sum(Box<Expression>, Box<Expression>),
	Sub(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Div(Box<Expression>, Box<Expression>),
	Pow(Box<Expression>, Box<Expression>),
	// Mod, In

	Eq(Box<Expression>, Box<Expression>),
	Neq(Box<Expression>, Box<Expression>),
	// Gt, Lt, Geq, Leq

	// LogAnd, LogOr, LogXor
	And(Box<Expression>, Box<Expression>),
	Or(Box<Expression>, Box<Expression>),

}