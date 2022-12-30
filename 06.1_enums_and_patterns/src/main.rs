#[derive(Debug)]
enum IpAddrKind {
  // values directly in the enum
  V4(String),
  V6(String),
}

#[derive(Debug)]
// not necessary when using enum!
struct IpAddr {
  _kind: IpAddrKind,
  _address: String,
}

impl IpAddr {
  fn _new(_kind: IpAddrKind, _address: &str) -> Self {
    Self {
      _kind,
      _address: String::from(_address),
    }
  }
}

// behaves similarly to NULL as in Option(5)
enum _Option<T> {
  None,
  Some(T),
} // std::Option is brought into scope by default

fn main() {
  let _ipv4 = IpAddrKind::V4;
  let _ipv6 = IpAddrKind::V6;

  // first struct format
  //let home = IpAddr::new(IpAddrKind::V4, "127.168.0.0");

  // typed enums allow to inject data directly and have their own constructors
  let home = IpAddrKind::V4(String::from("127.0.0.1"));
  println!("home: \n{:#?}", home);

  // 5 type is inferred from some(T)
  let _some_number = Some(5);
  // None type cannot be inferred, we need to specify its option
  let _null_value: Option<i32> = None;
}
