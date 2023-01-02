use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
  // error handling with match
  {
    let _greeting_file = match File::open("hello.txt") {
      Ok(file) => file,
      Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(file) => file,
          Err(e) => panic!("failed to create file {:?}", e),
        },
        other => {
          panic!("File::Open failed {:?}", other)
        }
      },
    };
  }

  // error handling with closure
  {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Problem creating the file: {:?}", error);
        })
      } else {
        panic!("Problem creating the file: {:?}", error);
      }
    });
  }

  // basic error handling, aborts on fail
  {
    let _greeting_file = File::open("hello.txt").unwrap();
    // for better error messages
    let _greeting_file = File::open("hello.txt").expect("file should exist");
  }
}

// long error handling
fn _read_username_from_file() -> Result<String, io::Error> {
  let file_result = File::open("hello.txt");
  let mut file = match file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };
  let mut username = String::new();

  match file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}

// shorter version
fn _read_username_from_file2() -> Result<String, io::Error> {
  // Question marks automatically return the error on fail
  let mut file = File::open("hello.txt")?;
  let mut username = String::new();
  file.read_to_string(&mut username)?;
  Ok(username)
}

// shorter shorter chaining
fn _read_username_from_file3() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;
  Ok(username)
}

// shorter shorter filesystem wrapper
fn _read_username_from_file4() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
