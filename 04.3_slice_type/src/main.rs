fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    /* byte literal sytax */
    if item == b' ' {
      return i;
    }
  }
  1
}

// str type works on both literals, strings and slices
fn first_slice(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    /* byte literal sytax */
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

fn main() {
  // reference of a temporary compiles :o
  println!("index {}", first_word(&String::from("Hello world!")));

  let str = String::from("hello world");

  // index is not tied to the originating string
  // if string mutates index is no longer valid
  let index = first_word(&str);
  println!("index {index}");

  //fixed by getting a reference to a portion of the string
  {
    let hello = &str[0..5];
    //same as
    let _hello = &str[..5];
    let world = &str[6..11];
    //same as
    let _world = &str[6..];
    //reference to the entire string
    let _helloworld = &str[..];
    println!("tied references: {} {}", hello, world);
    println!("returned slice: {}", first_slice(&str));
  }

  //slices work with arrays as well
  let arr = [1, 2, 3, 4, 5];
  let slice = &arr[1..3];
  assert_eq!(slice, &[2, 3]);
}
