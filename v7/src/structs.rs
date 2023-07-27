#[derive(PartialEq, Eq)]
pub struct Struct {
	extended: Vec<Box<Struct>>,
	name: Id,
	props: HashMap<Id, Box<Struct>>
}