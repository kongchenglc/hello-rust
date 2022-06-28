pub fn start() {
  struct_syntax();
  tuple_struct();
  get_area();
}

fn struct_syntax() {
  // struct 如果可变则所有字段可变 mut
  #[derive(Debug)]
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let email = String::from("xxx@xxx.com");

  let user1 = User {
    email,
    username: String::from("xxx"),
    active: true,
    sign_in_count: 121,
  };

  let use2 = User {
    email: String::from("another@xx.com"),
    ..user1 // borrow
  };

  println!("{:#?}", use2);
}

fn tuple_struct() {
  let a = (2, 21);
  struct Color(i32, i32, i32);
  let b = Color(0, 0, 0);
}

fn unit_like_struce() {
  struct A {} // 空struct用来挂trait
}

// fn struct_ownership() {}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn area(&self, times: u32) -> u32 {
    self.width * self.height * times
  }
}
impl Rectangle { // 可以有多个impl
  fn create(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
  }
}
fn get_area() {
  let a = Rectangle {
    width: 10,
    height: 20,
  };
  let b = Rectangle::create(20, 10);
  println!("{}", &&&&&&&&&&a.area(2)); // 自动解指针引用
  println!("{:?}", b);
}
