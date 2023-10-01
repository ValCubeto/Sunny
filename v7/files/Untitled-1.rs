Namespace = Struct {
	name: "Namespace",
	values: &[
		Variable {
			name: "name",
			instance_of: &String,
			value: Null
		}
	],
	methods: hashmap! {
		get_value: Builtin(|args: Arguments| {
			expect_args!(args, &[&String])
		})
	}
}


main = Instance {
	of: &Namespace,
	values: &["main"]
}