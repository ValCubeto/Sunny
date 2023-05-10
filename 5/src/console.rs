use std::collections::HashMap;

#[allow(unused)]
pub fn init() -> HashMap<&'static str, HashMap<&'static str, [&'static str; 2]>> {
	[
		(
			"colors",
			HashMap::<&'static str, [&'static str; 2]>::from([
				("red", ["31", "39"])
			])
		)
	].into()
}