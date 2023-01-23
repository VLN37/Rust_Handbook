use quick_sort::quick_sort;

fn main() {
  let mut arr = [3, 5, 2, 1];
  println!("{:#?}", arr);
  quick_sort(&mut arr);
  println!("{:#?}", arr);
}
