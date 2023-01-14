use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub file_path: String,
}

#[allow(dead_code)]
impl Config {
  // new should never fail, returns Self
  pub fn new(args: &[String]) -> Self {
    Self {
      query: args[1].clone(),
      file_path: args[2].clone(),
    }
  }

  // build may fail, returns Result
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Usage: cmd {{needle}} {{haystack path}}");
    }
    Ok(Self {
      query: args[1].clone(),
      file_path: args[2].clone(),
    })
  }
}

// dyn Error means a dynamic type that implements the type Error defined in std
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  println!("{:15} {}", "Searching for: ", config.query);
  println!("{:15} {}", "In file:", config.file_path);
  // trailing ? returns an Error value rather than panicking like expect would
  let content = fs::read_to_string(config.file_path.clone())?;
  println!("\nFile content\n\n{content}");
  Ok(())
}
