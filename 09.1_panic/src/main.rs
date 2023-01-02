/*
 * this tag on Cargo.toml prevents stack unwinding
 * [profile.release]
 * panic = 'abort'
 */

fn main() {
  let v = vec![1, 2, 3];
  // contrary to C, out of bounds instantly aborts
  v[99];
  panic!("crash");
}
