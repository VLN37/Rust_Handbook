#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // shorthand for self: &Self -> present in all impl as first parameter
  fn area(&self) -> u32 { self.width * self.height }

  // getters can have the same name as the field
  fn width(&self) -> u32 { self.width }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // functions without self are similar to non members
  fn new_square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  // usage of non member functions
  let _square = Rectangle::new_square(3);
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 20,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 25,
    height: 55,
  };

  println!("The area of the rectangle is {} square pixels.", rect1.area());
  println!("The area of the rectangle is {} square pixels.", rect1.width());
  println!();
  println!(
    "{} \n{:#?} \ncan hold   \n{:#?}\n",
    rect1.can_hold(&rect2),
    rect1,
    rect2
  );
  println!(
    "{} \n{:#?} \ncan't hold \n{:#?}\n",
    rect1.can_hold(&rect3),
    rect1,
    rect3
  );
}
