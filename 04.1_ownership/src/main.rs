// each value has an owner
// there can only be one owner
// when the owner goes out of scope the value is dropped

fn main() {
  {
    let _s = "hello";
  } // s is dropped

  // here both variables exist and are valid as they are both on the stack and
  // have implemented the COPY trait
  {
    let _x = 5;
    let _y = _x;
  }

  // here s1 is invalidated and its data pointer borrowed by s2
  {
    let _s1 = String::from("hello");
    let _s2 = _s1;
    println!("${_s2}");
    // does not compile
    // println!("${_s1}");
  }

  {
    let _s1 = String::from("hello");
    let mut _s2 = _s1.clone();
    _s2.push_str(", world!");
    println!("${_s1}");
    println!("${_s2}");
  } // when S goes out of scope memory is automatically deallocated with drop()
}

fn _calculate_len(s: String) -> (String, usize) {
  let len = s.len();
  (s, len)
} // transfers ownership of S and len, AkA: moves them
