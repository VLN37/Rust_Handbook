#![allow(unused)]

use anyhow::Context;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
  /// the pattern to look for
  pattern: String,
  /// haystack file
  path: std::path::PathBuf,
}

fn find_matches(file: &File, args: &Cli) {
  let content = BufReader::new(file);
  for result in content.lines() {
    let line = result.unwrap_or_else(|error| panic!("{:?}", error));
    if line.contains(&args.pattern) {
      println!("{}", line)
    }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  let args = Cli::parse();

  log::info!("Minigrep V2 has started");

  let file = File::open(&args.path)
    .with_context(|| format!("could not read file {:#?}", args.path))?;
  find_matches(&file, &args);

  log::info!("Minigrep V2 operation succesful");
  Ok(())
}
