mod front_of_house {
  // pub on both so outer functions can access the module and its details
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  // absolute as crate means root, AKA this file
  crate::front_of_house::hosting::add_to_waitlist();
  // same as which is relative
  front_of_house::hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // not allowed to see or modify the seasonal fruit that comes with the meal
  // meal._seasonal_fruit = String::from("blueberries");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
  println!("As the second course we would like {:?} and {:?}", order1, order2);
}

fn deliver_order() {}
mod back_of_house {

  fn cook_order() {}

  pub fn fix_incorrect_order() {
    cook_order();
    // here you are referencing a relative path from the parent (..)
    // this allows you to rearrange modules without correcting paths manually
    super::deliver_order();
  }

  pub struct Breakfast {
    pub toast: String,
    // since this field is private a public constructor is mandatory
    _seasonal_fruit: String,
  }

  impl Breakfast {
    // public constructor
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        _seasonal_fruit: String::from("peaches"),
      }
    }
  }

  #[derive(Debug)]
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub mod nested {
  pub fn nested_func() {}
}

pub fn waiter() { back_of_house::fix_incorrect_order(); }
