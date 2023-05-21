
#[allow(unused)]
macro_rules! parse_namespace {
	() => {{
		let namespace = HashMap::<String, Any>::new();
		namespace
	}}
}

#[allow(unused)]
pub(crate) use parse_namespace;