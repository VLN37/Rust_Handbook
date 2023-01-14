use minigrep;
use std::env;
use std::process;

#[allow(dead_code)]
fn parse_config(args: &[String]) -> minigrep::Config {
  let query = args[1].clone();
  let file_path = args[2].clone();
  minigrep::Config { query, file_path }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
    println!("Build failed:\n{}", err);
    process::exit(1);
  });

  if let Err(err) = minigrep::run(&config) {
    println!("Run failed:\n{}", err);
    process::exit(1);
  }
}
