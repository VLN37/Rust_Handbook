fn partition<T>(arr: &mut [T], low: usize, high: usize) -> usize
where
  T: Ord + std::fmt::Display
{
  let pivot = high;
  let mut store_index = low;
  let mut last_index = high;

  loop {
    while arr[store_index] < arr[pivot] {
      store_index += 1;
    }
    while last_index != 0 && arr[last_index] > arr[pivot] {
      last_index -= 1;
    }
    println!("val1 {}, val2 {}", arr[store_index], arr[last_index]);
    println!("low {}, high {}", store_index, last_index);
    if store_index >= last_index {
      break;
    } else {
      arr.swap(store_index, last_index);
    }
  }
  arr.swap(store_index, last_index);
  // if arr[store_index] == arr[last_index] {
  //   store_index += 1;
  // }
  store_index
}

fn recurse_sort<T: Ord>(arr: &mut [T], low: usize, high: usize)
where
  T: Ord + std::fmt::Display
{
  if low < high {
    let p = partition(arr, low, high);
    if p != 0 {
      recurse_sort(arr, low, p - 1);
    }
    recurse_sort(arr, p + 1, high);
  }
}

pub fn quick_sort<T: Ord>(arr: &mut [T])
where
  T: Ord + std::fmt::Display
{
  if arr.is_empty() {
    return;
  }
  let len = arr.len();
  recurse_sort(arr, 0, len - 1);
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn empty() {
    let arr1: [i32; 0] = [];
    let mut arr2: [i32; 0] = [];
    quick_sort(&mut arr2);
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn sorted() {
    let mut arr1 = [-42, 1, 3, 5, 9];
    let mut arr2 = [-42, 1, 3, 5, 9];

    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn inverted() {
    let mut arr1 = [9, 5, 4, 3, 1, -42];
    let mut arr2 = [9, 5, 4, 3, 1, -42];

    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn mix() {
    let mut arr1 = [2, 32, 4, 13, 1, -42, 132];
    let mut arr2 = [2, 32, 4, 13, 1, -42, 132];

    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }

  #[test]
  fn repeated() {
    let mut arr1 = [3, 5, 2, 2, 32, 1];
    let mut arr2 = [3, 5, 2, 2, 32, 1];

    quick_sort(&mut arr2);
    arr1.sort();
    assert_eq!(arr1, arr2);
  }
}
