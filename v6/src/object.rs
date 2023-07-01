use std::collections::HashMap;
use std::sync::Arc;
use crate::types::Value;

pub type Object = HashMap<Arc<str>, Value>;