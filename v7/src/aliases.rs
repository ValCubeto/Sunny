pub type Id = std::rc::Rc<str>;
#[allow(unused)]
pub type Dict = std::collections::HashMap<Id, crate::values::Value>;
pub type Arguments = Vec<crate::arguments::Argument>;
