pub fn start() {
  // scope();
  // string_type();
  // string_move();
  // string_clone();
  // copy_trait();
  // fun_ownership();
  // ref_borrow();
  // string_slice();
}

fn scope() {
  // s不可用
  let s = "hello";
  println!("{}", s)
} // s不可用

fn string_type() {
  // 字面量类型 是 &str: 栈内存
  // String类型: heap堆内存
  let mut s = String::from("hello");
  s.push_str(",world");

  // 走出作用域 s 自动调用 drop 函数
}

fn string_move() {
  // 防止double free
  let s = String::from("hello");
  let mut s2 = s;
  // s会被move到s2，s不可用
  // s.push_str(",world");
  s2.push_str(",world");
}

fn string_clone() {
  let mut s = String::from("hello");
  let mut s2 = s.clone(); // 深拷贝heap数据
  s.push_str(",world");
  s2.push_str(",world");
}

fn copy_trait() {
  // int float bool char tuple()
  let x = 5;
  let y = x;
  println!("{}{}", x, y)
}

fn fun_ownership() {
  // 函数参数传递以及返回值都会发生move
  let s = String::from("hello");
  let s2 = trans_ownership(s);
  // println!("{}", s); // 不行 所有权已经被移交
  println!("{}", s2); // 行

  fn trans_ownership(param: String) -> String {
    return param;
  }
}

// 引用或借用: 引用&(类似指针)
fn ref_borrow() {
  let mut s = String::from("hello");
  // 同一个作用域 不可同时定义多个可变引用防止数据竞争
  // let s2 = &mut s;
  // let s3 = &mut s;
  // println!("{}{}", s2, s3);

  // 多个作用域 或者 非同时 可以定义多个可变引用
  {
    let s2 = &mut s;
    println!("{}", s2);
  }
  let s3 = &mut s;
  println!("{}", s3);

  // 不可以同时有不可变和可变引用

  s.push_str(",world");
  let len = get_length(&mut s);
  println!("{}", len);

  fn get_length(s: &mut String) -> usize {
    s.push_str(",world");
    s.len()
  }
}

// 不被允许，已经drop的引用不能return到作用域外部
// fn dangle() -> &String {
//   let s = String::from("str");
//   &s // 作用域结束自动drop
// }

// 切片 slice
fn string_slice() {
  let s = String::from("hello, world");
  let hello = &s[..5];
  let world = &s[6..];
  let whole = &s[..]; // &str 常用在参数传递

  println!("{}{}{}", hello, world, whole)
}
