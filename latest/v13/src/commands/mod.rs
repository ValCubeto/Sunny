use crate::args::ParsedArgs;
use crate::terminal::Stylize;

use_item!(help, version, run, init);

pub fn run_cmd(args: ParsedArgs) {
  match args.command.as_str() {
    "help" => help(),
    "version" => version(),
    "run" => run(args),
    "init" => init(),
    "" => help(),
    other => println!("{}: {other:?}", "No such command".error()),
  }
}
