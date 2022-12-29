fn main() {
  // control structures operate ONLY on boolean
  {
    let i = 42;
    // if i   -> does not compile
    if i != 0 {
      // braces mandatory
      println!("number is not zero");
    }
  }

  // if is an expression
  {
    let condition = true;
    // both arms of the if expression must be of the same type
    let _number = if condition { 42 } else { 0 };
  }

  // loops
  {
    let mut counter = 0;

    let result = loop {
      counter += 1;
      if counter == 21 {
        // returns to the loop callee
        break counter * 2;
      }
    }; // semicolon ends the loop statement
    println!("the result is {result}");
  }

  // loop labels
  {
    let mut count = 0;
    //loop label syntax
    'counting_up: loop {
      println!("{:10} = {count}", "count");

      let mut remaining = 10;

      loop {
        println!("{:10} = {remaining}", "remaining");
        if remaining == 0 {
          break;
        }
        if count == 2 {
          // break applies on the innermost loop unless labeled
          break 'counting_up;
        }
        remaining -= 1;
      }
      count += 1;
    }
  }

  // looping collections
  {
    let arr = [10, 20, 30, 40, 50];

    //with while loop
    let mut i = 0;
    while i < 5 {
      println!("{:10} = {}", "index[i]", arr[i]);
      i += 1;
    }

    //with iterators
    for element in arr {
      println!("{:10} = {element}", "element");
    }

    //with range support
    for number in (1..4).rev() {
      println!("{number}");
    }
    println!("LIFTOFF!");
  }
}
