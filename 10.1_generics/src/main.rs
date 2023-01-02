fn _typed_largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// here we need to specify T is orderable by applying the recipe to types that
// implement the PartialOrd trait, thus enabling comparisons
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// you can use generics in enums and structs as well, as these std library impls
enum _Result<T, E> {
  Ok(T),
  Err(E),
}

enum _Option<T> {
  Some(T),
  None,
}

// generic methods reference
#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn getx(&self) -> &T { &self.x }
}

impl<T> Point<T> {
  fn gety(&self) -> &T { &self.y }
}

fn main() {
  let numbers = vec![1, 2, 4, 8, 16, 32];
  println!("largest value: {}", largest(&numbers));
  let chars = vec!['a', 'b', 'd', 'q', 'y', 'c'];
  println!("largest value: {}", largest(&chars));

  let var = Point { x: 21, y: 42 };
  println!("Point   {:?}", var);
  println!("Point x {}\nPoint y {}", var.getx(), var.gety());
}
