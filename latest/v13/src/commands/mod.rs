use crate::args::ParsedArgs;
use crate::use_fn;

use_fn!(help, version, run);

pub fn run_cmd(args: ParsedArgs) {
  match args.command.as_str() {
    "help" => help(),
    "version" => version(),
    "run" => run(args),
    _ => help(),
  }
}
