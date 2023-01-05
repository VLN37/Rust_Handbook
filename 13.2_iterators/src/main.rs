fn main() {
  // iterating through a list
  {
    let vec = vec![1, 2, 3];

    let it = vec.iter();
    // for loop makes it mutable by itself
    for val in it {
      println!("Val: {val}");
    }
  }

  // iterators and maps
  {
    let vec = vec![1, 2, 3];
    // since map returns an iterator to a new vector
    // you have to collect it prior to atribution
    let mapped_vector: Vec<_> = vec.iter().map(|x| x + 1).collect();
    for val in mapped_vector.iter() {
      println!("mapped Val: {val}");
    }
  }
}
