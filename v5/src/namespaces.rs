#[allow(unused)]
macro_rules! collect_namespace {
	() => {{
		let exports = HashMap::<String, Any>::new();
	}}
}

#[allow(unused)]
macro_rules! collect_main {
	() => {{
		
	}}
}

#[allow(unused)]
pub(crate) use collect_namespace;

#[allow(unused)]
pub(crate) use collect_main;