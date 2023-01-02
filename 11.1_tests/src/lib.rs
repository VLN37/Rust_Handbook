pub fn add(a: usize, b: usize) -> usize {
  a + b
}

#[derive(PartialEq, Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_test() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  // we can specify a substring of what should have panicked
  #[should_panic(expected = "fine")]
  fn failure() {
    panic!("everything is fine");
  }

  #[test]
  fn can_hold_smaller() {
    let larger = Rectangle {
      width: 10,
      height: 10,
    };
    let smaller = Rectangle {
      width: 5,
      height: 5,
    };
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn can_hold_larger() {
    let larger = Rectangle {
      width: 10,
      height: 10,
    };
    let smaller = Rectangle {
      width: 5,
      height: 5,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn not_equal() {
    assert_ne!(add(2, 3), 7);
  }

  #[test]
  fn equal() {
    assert_eq!(add(2, 2), 4, "this has a custom message on failure");
  }

  #[test]
  #[ignore]
  fn fail() {
    assert_eq!(add(2, 2), 5, "this has a custom message on failure");
  }
}
