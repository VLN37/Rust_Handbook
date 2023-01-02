// since borrow checker does not know whether we return a or b
// we need to specifically annotate the references
// they will be checked against the one that lives the shortest
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// references within structs must respect lifetimes and annotate them
struct HoldsReference<'a> {
  excerpt: &'a str,
}

fn main() {
  let str1 = String::from("tiny string is tiny");
  let str2 = String::from("long string is longer");

  println!("the longest is: {}", longest(&str1, &str2));
  let reference = HoldsReference {
    excerpt: str2.split(' ').next().unwrap(),
  };

  // str1 needs to live as long as the refence we took in excerpt
  println!("{}", reference.excerpt);
}
