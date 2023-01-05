pub trait Iterator {
  // associated type
  type Item;

  // iterators has real value until we call next on it, returning the next item
  fn next(&mut self) -> Option<Self::Item>;
}

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod stuff {
  use super::*;

  #[test]
  fn iterator() {
    let vec = vec![1, 2, 3];

    let it = vec.iter();
    // for loop makes it mutable by itself
    for val in it {
      println!("Val: {val}");
    }
  }

  #[test]
  fn iterator_next() {
    let vec = vec![1, 2, 3];
    let mut it = vec.iter();

    // calling next returns a Some struct for validation
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
    // into_iter() takes ownership
    // iter_mut() allows change
  }

  #[test]
  fn iterator_consumption() {
    let vec = vec![1, 2, 3];

    let it = vec.iter();
    // after this line the iterator points to none, it is fully consumed
    let total: i32 = it.sum();
    assert_eq!(total, 6);
  }

  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 13,
        style: String::from("sandal"),
      },
      Shoe {
        size: 15,
        style: String::from("boot"),
      },
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(
      in_my_size,
      vec![Shoe {
        size: 10,
        style: String::from("sneaker")
      }]
    );
  }
}
