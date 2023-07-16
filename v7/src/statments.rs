use crate::{id::Id, arguments::Arguments};

#[derive(Debug)]
pub enum Statment {
	Call(Id, Arguments)
}