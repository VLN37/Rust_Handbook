// traits work similarly to interfaces, a type must impl its method signatures
pub trait Summary {
  // you can also define a default impl that can be overridden later
  fn summarize(&self) -> String { String::from("(Read more...)") }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub struct RedditPost {
  pub username: String,
  pub title: String,
  pub content: String,
  pub replies: i32,
}
// uses default implementation
impl Summary for RedditPost {
  fn summarize(&self) -> String {
    format!(
      "username: {}\ntitle: {}\ncontent: {}\n",
      self.username, self.title, self.content
    )
  }
}

// users of the library would need to bring both into scope, i.e.:
// use aggregator::{Summary, Tweet};

// you can accept parameters types that implement a certain trait
pub fn notify(item: &impl Summary) {
  println!("New notication!\n{}", item.summarize());
}

// longer version
pub fn notify_longer<T: Summary>(item: &T) {
  println!("New notication!\n{}", item.summarize());
}

// you can require multiple implementations for the parameter
pub fn _notify_multiple(item: &(impl Summary + Display)) {
  println!("New notication!\n{}", item.summarize());
}

// to make trait bounds less cluttered we can use where
pub fn _notify_multiple_sugar<T>(item: &T)
where
  T: Display + Summary,
{
  println!("New notication!\n{}", item.summarize());
}

// use abstractions as return values
fn _returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

use core::fmt::Display;
fn main() {
  let tweet = Tweet {
    username: "dood".to_string(),
    content: "i'm pretentious".to_string(),
    reply: false,
    retweet: false,
  };
  println!("A tweet:\n {}\n", tweet.summarize());

  let post = RedditPost {
    username: "vTuber".to_string(),
    title: "check out my new Vdeo".to_string(),
    content: "*VR chat intensifies*".to_string(),
    replies: 7,
  };
  println!("New reddit post!\n{}", post.summarize());
  notify(&tweet);
  notify_longer(&tweet);
}
