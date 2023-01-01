fn main() {
  use std::collections::HashMap;

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name).copied().unwrap_or(0);
  println!("{}", score);

  // hash_map iteration
  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  // insert takes ownership of the strings, invalidating the past variables
  // if we inserted references they need to live past the lifetime of the map
  map.insert(field_name, field_value);
}
