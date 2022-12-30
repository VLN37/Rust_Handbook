struct User {
  username: String,
  _active: bool,
  _email: String,
  _sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
  User {
    username, // key as value
    _email: email,
    _active: true,
    _sign_in_count: 1,
  }
}

struct _Color(u8, u8, u8);

fn main() {
  // the entire struct must be mutable
  let mut _user1 = User {
    _email: String::from("me@domain.com"),
    _active: true,
    _sign_in_count: 1,
    username: String::from("dood"),
  };

  // constructor approach
  let _user2 = build_user(
    String::from("pwn@place.com"),
    String::from("pwnz0r"),
  );

  // partial copy approach
  let _user3 = User {
    _email: String::from("rekt@boom.com"),
    .._user1 // this invalidates user1 since username is moved into user3
  };

  _user1._email = String::from("m3@domain.com");
  println!("{:?}", _user3.username);

  let _white = _Color(255, 255, 255);
  println!("R{} G{} B{}", _white.0, _white.1, _white.2);

  // struct tuple syntax
}
