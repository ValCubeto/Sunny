
#[cfg(test)]
mod test {
	use crate::hashmap;
	#[test]
	fn maps() {
		let map = hashmap! {
			test => "Hola"
		};
		let map2 = std::collections::HashMap::from([
			(std::rc::Rc::<str>::from("test"), "Hola")
		]);
	}
}