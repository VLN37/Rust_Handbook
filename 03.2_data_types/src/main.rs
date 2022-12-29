fn main() {
  {
    // rust stores characters as Unicode Scalar Value // more on chapter 8
    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';
  }

  {
    // struct initialization
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // struct destructuring
    let (x, y, z) = tup;
    let width = 20;
    // $ skips value in the variadic parameters list
    println!("{:width$} -> {x}", "value of x");
    println!("{:20} -> {}", "value of tup.0", tup.0);
    println!("{:width$} -> {y}", "value of y");
    println!("{:20} -> {}", "value of tup.1", tup.1);
    println!("{:width$} -> {z}", "value of z");
    println!("{:20} -> {}", "value of tup.2", tup.2);
  }

  {
    // explicit array type declaration
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];

    // constructor of array with 3 elements initialized to 5
    let mut _arr = [3; 5];

    // :? triggers use of std::fmt::Debug rather than Display
    println!("{:?}", _arr);

    // element access
    _arr[1] = 2;
  }
}
