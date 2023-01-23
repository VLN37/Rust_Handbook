fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
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
    if store_index >= last_index {
      break;
    } else {
      arr.swap(store_index, last_index);
    }
  }
  // arr.swap(store_index, last_index);
  store_index
}

fn recurse_sort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
  if low < high {
    let p = partition(arr, low, high);
    recurse_sort(arr, low, p - 1);
    recurse_sort(arr, p + 1, high);
  }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  let len = arr.len() - 1;
  recurse_sort(arr, 0, len);
}

fn main() {
  let mut arr = [3, 5, 2, 1];
  println!("{:#?}", arr);
  quick_sort(&mut arr);
  println!("{:#?}", arr);
}
