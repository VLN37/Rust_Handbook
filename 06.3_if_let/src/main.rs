// new switch as in the match_control example
fn _config_max() {
  let config_max = Some(3u8 /* wtf??? */);
  match config_max {
    Some(max) => println!("The maximum is configured to {}", max),
    _ => (), // shorthand for do nothing
  }
}

// using if let allows you to ommit the weird None case
// loses comprehensiveness but fits well in an apply or ignore pattern
fn config_max() {
  let config_max = Some(42u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to {}", max);
  }
}

fn main() { config_max(); }
