use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

#[allow(dead_code)]
impl Config {
  // new should never fail, returns Self
  pub fn new(args: &[String]) -> Self {
    Self {
      query: args[1].clone(),
      file_path: args[2].clone(),
      ignore_case: false,
    }
  }

  // build may fail, returns Result
  pub fn build(
    // receives anything that impl the Iterator trait, as std::env::args do
    // more in Chapter 10: traits as parameters
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
    args.next(); // filename

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("No needle provided")
    };
    let file_path = match args.next() {
      Some(path) => path,
      None => return Err("No haystack provided")
    };
    Ok(Self {
      query,
      file_path,
      ignore_case: env::var("IGNORE_CASE").is_ok(),
    })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

// dyn Error means a dynamic type that implements the type Error defined in std
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  // trailing ? returns an Error value rather than panicking like expect would
  let content = fs::read_to_string(&config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &content)
  } else {
    search(&config.query, &content)
  };

  for line in results {
    println!("{line}");
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "silent";
    let contents = "\
Deep silent complete,
black velvet sea,
the sirens are calling for me.";
    assert_eq!(vec!("Deep silent complete,"), search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "dEEP";
    let contents = "\
Deep silent complete,
black velvet sea,
the sirens are calling for me.";
    assert_eq!(
      vec!("Deep silent complete,"),
      search_case_insensitive(query, contents)
    );
  }
}
