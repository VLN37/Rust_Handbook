use quick_sort::quick_sort;

fn main() {
  let mut arr1 = [3, 5, 2, 2, 32, 1];
  let mut arr2 = [3, 5, 2, 2, 32, 1];

  quick_sort(&mut arr2);
  arr1.sort();
  println!("{:#?}", arr2);
}
