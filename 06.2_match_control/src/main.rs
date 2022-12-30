#[derive(Debug)]
enum UsState {
  _Alabama,
  Alaska,
}

enum Coin {
  _Penny,
  _Nickel,
  _Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  // matches need to cover all cases
  match coin {
    Coin::_Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::_Nickel => 5,
    Coin::_Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  // using default Option<T> that was brought into scope
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn roll_dice() {
  let dice_roll = 7;
  match dice_roll {
    7 => println!("Lucky seven!"),
    _other => move_player(_other),
  }
}

fn move_player(_qty: i32) {}

fn main() {
  value_in_cents(Coin::Quarter(UsState::Alaska));
  let five = Some(5);
  let _six = plus_one(five);
  let _none = plus_one(None);
  roll_dice();
}
