use std::{rc::Rc, fmt::Debug};

#[derive(Clone)]
pub struct Id(pub Rc<str>);

impl Debug for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl From<&str> for Id {
	fn from(value: &str) -> Self {
		Id(Rc::from(value))
	}
}

#[allow(unused)]
impl Id {
	pub fn to_str(&self) -> &str {
		&(self.0)
	}
}