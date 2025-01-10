use_item!(help, version, run, init);
use crate::args::ParsedArgs;

pub fn run_cmd(args: ParsedArgs) {
  match args.command.as_str() {
    "help"  | "" => help(),
    "version" => version(),
    "run" => run(args),
    "init" => init(),
    other => argument_err!("no such command {other:?}"),
  }
}
