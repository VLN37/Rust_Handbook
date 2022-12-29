// parameters MUST be typed
fn a_function(x: i32) {
  println!("This is a function. Its argument is ${x}");
}

// statements vs expressions
fn caller_and_statements() -> i32 /* return type */ {
  //  statement = statement -> invalid
  //      let x = (let y = 6);

  /* y is a caller */
  let y = {
    let x = 41;
    // expressions have no semicolon, returns value to the caller
    x + 1
  };
  y
}

fn main() {
  a_function(42);
  caller_and_statements();
  println!("Hello, world!");
}
