pub type Id = std::rc::Rc<str>;
pub type Dict = std::collections::HashMap<Id, crate::values::Value>;
pub type Arguments = Vec<Argument>;