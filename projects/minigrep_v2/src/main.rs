#![allow(unused)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use std::fs;
use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Cli {
  /// the pattern to look for
  pattern: String,
  /// haystack file
  path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();

  let file = File::open(&args.path)
    .with_context(|| format!("could not read file {:#?}", args.path))?;

  let content = BufReader::new(file);
  for result in content.lines() {
    let line = result.unwrap_or_else(|error| panic!("{:?}", error));
    if line.contains(&args.pattern) {
      println!("{}", line)
    }
  }
  Ok(())
}
