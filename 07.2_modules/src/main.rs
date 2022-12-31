// may be defined inline or defaulted to one of the following:
// 1 - root/garden.rs
// 2 - root/garden/main.rs
// 3 - root/garden/mod.rs
mod garden;

fn main() {
  println!("Package root");
  crate::garden::garden_module();
  // crate::garden::vegetables::vegetables_module();
}
