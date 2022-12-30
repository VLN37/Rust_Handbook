// references borrow ownership, they do not invalidate the previous variable
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

fn main() {
  // references themselves also have to be mutable
  {
    let mut var = String::from("hello");
    println!("{:15} -> {var}", "string before");
    change(&mut var);
    println!("{:15} -> {var}", "string after");
  }

  // you cannot have two mutable references to the same variable
  // you can only have one mutable reference or as many immutables as you want
  {
    let mut s = String::from("hello");

    // this is invalid
    {
      let _r1 = &mut s;
      // let r2 = &mut s;
    }

    // this is invalid
    {
      //valid to HAVE
      let _r1 = &s;
      let _r2 = &s;
      let _r3 = &mut s;
      // invalid to USE
      // println!("{_r1}");
    }

    {
      let _r1 = &mut s;
      println!("{:15} -> {}", "string before", _r1);
      change(&mut s);
    }
    {
      let _r2 = &mut s;
      println!("{:15} -> {}", "string after", _r2);
    }
  }
}
