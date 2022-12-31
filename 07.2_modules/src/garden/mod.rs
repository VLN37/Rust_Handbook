// this is the root of the garden crate

// mods or private for its parents, public for its children
// unless prefixed with pub
mod vegetables;
mod folderless_mod;

pub mod public {
  fn _func() {
    println!("public mod is public to external crates that use this module");
    println!("func, however, is not accessible");
  }
}

mod private {
  pub fn _func() {
    println!("private is not accessible to parent crates");
    println!("func, however, is public and accessible to itself and children");
  }
}

// pub functions are available
pub fn garden_module() {
  println!("the garden module");
  vegetables::vegetables_module();
  folderless_mod::function();
}
