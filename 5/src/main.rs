use ::std::{collections::HashMap, process::exit};

mod parse_args;
mod about;
mod std;

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file, args): _ = parse_args::parse();
	let args = args.iter().map(|v| Any::String(v.clone())).collect();
	let flags = flags.iter().map(|v| Any::String(v.clone())).collect();

	// let std = std::init();

	use toml::Table;

	let value: Table = match "foo = \n'bar'".parse::<Table>() {
		Ok(data) => {
			data
		},
		Err(err) => {
			eprintln!("erro {}", err);
			exit(1);
		}
	};

	dbg!(&value);

	#[allow(unused)]
	#[derive(Debug)]
	enum Any {
		None,
		Infinity,
		List(Vec<Any>),
		Dict(Dict),
		String(String),

		U8(u8),
		U16(u16),
		U32(u32),
		U64(u64),
		U128(u128),
		Usize(usize),
		I8(i8),
		I16(i16),
		I32(i32),
		I64(i64),
		I128(i128),
		Isize(isize),
		F32(f32),
		F64(f64),

		Number(String),
	}

	type Dict<T = Any> = HashMap<String, T>;

	#[allow(unused)]
	let mut global: Dict = Dict::from([
		("process".to_string(), Any::Dict(Dict::from([
			("exec_path".to_string(), Any::String(exec_path)),
			("flags".to_string(), Any::List(flags)),
			("file".to_string(), Any::String(file)),
			("args".to_string(), Any::List(args))
		]))),
		("none".to_string(), Any::None),
		("infinity".to_string(), Any::Infinity),
	]);

	println!("global = {:?}", global);
}