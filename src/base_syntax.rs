fn let_syntax() {
  let mut x = 5;
  println!("{}", x);
  x = 4;
  println!("{}", x);
}

const CONST_NUM2: i32 = 5;
fn const_syntax() {
  const CONST_NUM: i32 = 5;
  println!("{},{}", CONST_NUM, CONST_NUM2);
}

fn shadowing_syntax() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("{}", x);
  let x = "x";
  println!("{}", x);
}

fn type_syntax() {
  let a: u32 = "42".parse().expect("not a number");
  println!("{}", a);
}

fn signed_and_ungigned_int_syntax() {
  // i8    u8
  // i16   u16
  // i32   u32
  // i64   u64
  // i128  u128
  // isize usize  // 电脑架构x64 => i64 | u64
  let a = 57u64;
  let b = b'A';

  let c = 98_222;
  let d = 0xdd;
  let d = 0xddu8;
  // default i32
  println!("{}{}{}{}", a, b, c, d);
}

fn f_syntax() {
  let y = 2;
  let x = 2.8;
  // default f64
  let z = x as i32 + y; // 2
  println!("{}", z);
}

fn tuple_syntax() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("{}{}{}", tup.0, y, z)
}

fn arr_syntax() {
  let x = [1, 2, 3];
  let mut a = [3; 5];
  a[0] = 2;
  println!("{}", x[a[1]]); // error
}

fn vector_syntax() {}

pub fn start() {
  let_syntax();
  const_syntax();
  shadowing_syntax();
  type_syntax();
  signed_and_ungigned_int_syntax();
  f_syntax();
  tuple_syntax();
  arr_syntax();
  vector_syntax();
}
