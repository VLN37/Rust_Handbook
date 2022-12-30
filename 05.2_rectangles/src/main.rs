fn area(width: u32, height: u32) -> u32 {
  return width * height;
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn struct_area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

fn main() {
  // value style
  {
    let width1 = 30;
    let height1 = 50;

    println!("Area = {}", area(width1, height1));
  }

  // tuple style
  {
    let rect1 = (30, 50);
    println!("Area = {}", tuple_area(rect1));
  }

  // struct style
  {
    let rect1 = Rectangle {
      width: 10,
      height: 30,
    };
    println!("Area  = {}", struct_area(&rect1));
    println!("Debug = {:#?}", rect1);
    dbg!(&rect1);
  }
}
