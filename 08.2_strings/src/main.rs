// strings are wrappers for vec<char>

fn main() {
  let data = "initial contents";
  // literal conversion conversion
  let _str = data.to_string();
  // any trait that implements display can be converted to string
  let _str = "initial contents".to_string();
  // constructor method
  let _str = String::from("initial contents");

  // strings are UTF-8 encoded, containing characters such as
  let _hello = String::from("السلام عليكم");
  let _hello = String::from("Dobrý den");

  // concatenation
  let mut s1 = String::from("foo");
  let s2 = "bar";
  // push_str receives a slice so s2 does not lose ownership
  s1.push_str(s2);
  println!("s2 is {s2}");

  // but operator + takes ownership
  let _s1 = String::from("Hello, ");
  let _s2 = String::from("world!");
  // note s1 has been moved here and can no longer be used
  // concatenation requires an actual string on the left side
  let _s3 = _s1 + &_s2;
  println!("{_s2}");

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // only s1 loses ownership
  let _s = s1 + "-" + &s2 + "-" + &s3;
  // println!("{s1} {s2} {s3}");

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // no ownership loss
  let _s = format!("{s1}-{s2}-{s3}");
  println!("{s1} {s2} {s3}");

  // string length
  // since you cannot trust a 'character' to have one byte you cannot index them
  let hello = String::from("Hola");
  println!("size of str: {}", hello.len());
  let hello = String::from("Здра");
  println!("size of str: {}", hello.len());

  // you can iterate with chars so it actually returns a 2 byte representation
  for c in "Зд".chars() {
    println!("{c}");
  }
  // or you can iterate over bytes themselves as u8
  for b in "Зд".bytes() {
    println!("{b}");
  }
}
