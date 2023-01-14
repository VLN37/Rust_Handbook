use minigrep;
use std::env;
use std::process;

#[allow(dead_code)]
fn parse_config(args: &[String]) -> minigrep::Config {
  let query = args[1].clone();
  let file_path = args[2].clone();
  minigrep::Config { query, file_path, ignore_case: false }
}

fn main() {
  let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Build failed:\n{}", err);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(&config) {
    eprintln!("Run failed:\n{}", err);
    process::exit(1);
  }
}
