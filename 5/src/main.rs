// mod parsed_args;
mod info;
mod console;

fn main() {
	#[allow(unused)]
	// let (executor_path, flags, file_path, args): (String, Vec<String>, String, Vec<String>) = parsed_args::new();
	let make_color = |color_code: &str| {
		format!("\u{1b}[{}m", color_code)
	};
	let [red, red_end]: [String; 2] = console::init()["colors"]["red"].map(|color: &str| make_color(color));
	println!("{}SyntaxError{}: que", red, red_end);
}