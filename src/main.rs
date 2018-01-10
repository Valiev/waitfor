#[warn(dead_code)]
#[warn(unused_variables)]

#[macro_use]
extern crate clap;
use clap::{ App, ArgMatches };

use std::process::{ exit };
use std::io::{ self, BufRead };

type ActionResult = Result<(), String>;

fn string_action(args: &ArgMatches) -> ActionResult {
  let text = args.value_of("text").unwrap();
  let max_times = match args.value_of("times") {
    None => 1,
    Some(value) => {
      match value.parse::<usize>() {
        Ok(n) => n,
        Err(_e) => return Err("Failed to parse `--times` options".to_string()),
      }
    },
  };

  let mut counter = 0;
  let mut input = String::new();
  let stdin = io::stdin();

  loop {
    if counter >= max_times { break };
    // Need to clean up buffer, becauuse `read_line` appends collected data
    input.clear();
    match stdin.lock().read_line(&mut input) {
      Ok(n) => {
        if n==0  {
          if counter == 0 {
            return Err(format!("No `{}` occurence found", text).to_string());
          } else {
            return Err(format!("Unable to find {} occurences of `{}`", max_times, text).to_string());
          }
        }

        let occ: Vec<&str> = input.matches(text).collect();
        counter += occ.len();

        if occ.len() > 0 {
          print!("{}", input);
        };
      },
      Err(_) => {
        return Err("Failed to read `stdin`".to_string());
      }
    }
  }

  return Ok(());
}


fn run_command(matches: ArgMatches) -> ActionResult {
  let subcommand = matches.subcommand();
  match &subcommand {
    &("string", sub_args) => {
      let args = sub_args.unwrap();
      string_action(args)
    },
    &("regexp", sub_args) => {
      panic!("Not implemented yet");
    },
    _ => Err("No suitable subcommand".to_string()),
  }
}


fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  exit(match run_command(matches) {
    Ok(_) => 0,
    Err(err) => {
      eprintln!("{}", err);
      1
    }
  });
}
