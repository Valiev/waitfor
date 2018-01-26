use clap::{ ArgMatches };

pub fn has_arg(args: &ArgMatches, arg: &'static str) -> bool {
  match args.occurrences_of(arg) {
    0 => false,
    _ => true
  }
}

