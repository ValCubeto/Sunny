use_item!(help, version, run, init, _test_);
use crate::args::ParsedArgs;

pub fn run_cmd(args: ParsedArgs) {
  match args.command.as_str() {
    "help"  | "" => help(),
    "version" => version(),
    "run" => run(args),
    "init" => init(),
    "_test_" => _test_(args),
    other => argument_err!("no such command {other:?}"),
  }
}
