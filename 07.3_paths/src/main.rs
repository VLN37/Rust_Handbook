
// here this binary crate is using the lib defined in the same package
// as if you were the client of an API defined in the library
fn main() {
  paths::eat_at_restaurant();
  paths::waiter();
  paths::nested::nested_func();

  // condense deep paths
  use paths::nested;
  nested::nested_func();
  // rename scopes with as
  use paths::nested as scope;
  scope::nested_func();
  // you can also make the use statement available elsewhere
  pub use paths::nested as scope2;
  // you can bring everything publicly exported into scope with
  // use paths::*;
  use paths::nested::nested_func;
  nested_func();
}
