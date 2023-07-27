pub struct Instance {
	parent: Box<Struct>,
	values: HashMap<(Box<Struct>, Value)>
}