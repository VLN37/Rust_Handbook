fn main() {
  //macros
  {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 20;
    println!("The value of macro is {THREE_HOURS_IN_SECONDS}");
  }

  // mutable variables
  {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
  }

  //variable shadowing
  {
    let x = 2;
    //works only with let, mut doesn't compile
    let x = x * 2;
    println!("The value of shadow x is {x}");
  }

  //shadowing allows reusing names
  {
    // this creates new variables with new times
    let _spaces: &str = "  ";
    let _spaces: usize = _spaces.len();

    //this doesn't compile due to type checking
    // let _spaces: &str = "  ";
    // _spaces: usize = _spaces.len();
  }
}
