use crate::aliases::Id;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	None,
	Infinity,
	String(String),
	Id(Id),
	u8(u8)
}