use std::thread;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    // anonymous function that captures its scope
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };
  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!("User likes {:?} and gets {:?}", user_pref1, giveaway1);

  let user_pref1 = None;
  let giveaway1 = store.giveaway(user_pref1);
  println!("User likes {:?} and gets {:?}", user_pref1, giveaway1);

  closure_syntax();
}

#[allow(dead_code)]
fn closure_syntax() {
  fn  _add_one_1   (x: u32) -> u32 {x + 1}
  let _add_one_2 = |x: u32| -> u32 {x + 1};
  let _add_one_3 = |x: u32|             {x + 1};
  let _add_one_4 = |x: u32|              x + 1 ;

  let example = |x| x;

  let _s = example(String::from("hello"));
  // only one type can be inferred for each closure
  // let n = example(5);

  let mut list = vec![1, 2, 3];
  let mut mut_borrow = || list.push(7);
  // between definition and usage the mutable reference persists
  mut_borrow();
  println!("{:?}", list);
  // here you can't borrow mutably on 70 and immutably on 68
  // mut_borrow();

  // you can force ownership with move
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  thread::spawn(move || println!("From thread: {:?}", list))
      .join()
      .unwrap();
}
