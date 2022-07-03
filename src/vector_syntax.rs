// vector 是heap连续内存，push超长需要重新开辟heap。 array是固定大小
pub fn start() {
  push_get();
  borrow_and_get();
  map_vector();
}

fn push_get() {
  let mut v = Vec::new();

  let v2 = vec![1, 2, 3];

  v.push(1);
  v.push(2);

  let thrid = v[1]; // 下标过大会编译不过
  println!("{}", thrid);

  match v.get(100) {
    Some(value) => println!("{}", value),
    None => println!("none"),
  };
}

fn borrow_and_get() {
  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  // v.push(6); // x 可能超长需要新开辟空间，地址会改变
  println!("{}", first);
}

fn map_vector() {
  let mut v = vec![1, 2, 3];

  for i in &mut v {
    *i += 50;
    println!("{}", i)
  }
  for i in v {
    println!("{}", i)
  }
}


// 已知数量的类型，可以存放在vector里
fn vector_enum() {
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(1.1),
    SpreadsheetCell::Text(String::from("xx")),
  ];
}