#[derive(Debug)]
enum Robot {
  Id(i32),
  Name(String),
}

fn main() {
  let mut v: Vec<i32> = Vec::new();

  for i in 1..10 {
    v.push(i);
  }
  println!("{:?}", v);

  // exception support
  let value: Option<&i32> = v.get(2);
  match value {
    Some(value) => println!("Value is {value}"),
    None => println!("Index out of bounds"),
  }

  // multi type vectors!
  let mut workers = vec![Robot::Id(42)];
  workers.push(Robot::Name(String::from("GlaDOS")));

  // this & prevents move semantics
  // however, holding it prevents mutability of the whole vector
  for elem in &workers {
    println!("{:?}", elem);
  }

  // so the values are still existing here
  println!("{:?}", workers);
}
