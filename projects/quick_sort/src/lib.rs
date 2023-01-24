fn partition<T>(arr: &mut [T], low: usize, high: usize) -> usize
where
  T: Ord + std::fmt::Display
{
  let pivot = high;
  let mut i = low;

  for j in low..high {
    if arr[j] <= arr[pivot] {
      arr.swap(i, j);
      i += 1;
    }
  }
  arr.swap(i, high);
  i
}

fn fix_first<T>(arr: &mut [T])
where
  T: Ord + std::fmt::Display
{
  let mut min = 0;
  for i in 0..arr.len() {
    if arr[i] < arr[min] {
      min = i;
    }
  }
  if min > 0 {
    arr.swap(min, 0);
  }
}

fn recurse_sort<T>(arr: &mut [T], low: usize, high: usize)
where
  T: Ord + std::fmt::Display
{
  if low < high {
    let p = partition(arr, low, high);
    recurse_sort(arr, low, p - 1);
    recurse_sort(arr, p + 1, high);
  }
}

pub fn quick_sort<T: Ord>(arr: &mut [T])
where
  T: Ord + std::fmt::Display
{
  let len = arr.len();
  if len == 0 {
    return;
  }
  fix_first(arr);
  recurse_sort(arr, 1, len - 1);
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn empty() {
    let arr1: [i32; 0] = [];
    let mut arr2: [i32; 0] = [];

    println!("arr1: {:#?}", arr1);
    println!("arr2: {:#?}", arr2);
    quick_sort(&mut arr2);
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn sorted() {
    let mut arr1 = [-42, 1, 3, 5, 9];
    let mut arr2 = [-42, 1, 3, 5, 9];

    println!("arr1: {:#?}", arr1);
    println!("arr2: {:#?}", arr2);
    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn inverted() {
    let mut arr1 = [9, 5, 4, 3, 1, -42];
    let mut arr2 = [9, 5, 4, 3, 1, -42];

    println!("arr1: {:#?}", arr1);
    println!("arr2: {:#?}", arr2);
    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn mix() {
    let mut arr1 = [2, 32, 4, 13, 1, -42, 132];
    let mut arr2 = [2, 32, 4, 13, 1, -42, 132];

    println!("arr1: {:#?}", arr1);
    println!("arr2: {:#?}", arr2);
    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn repeated() {
    let mut arr1 = [3, 5, 2, 2, 32, 1];
    let mut arr2 = [3, 5, 2, 2, 32, 1];

    println!("arr1: {:#?}", arr1);
    println!("arr2: {:#?}", arr2);
    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }
}
