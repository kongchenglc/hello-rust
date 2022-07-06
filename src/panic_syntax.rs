use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

pub fn start() {
  // panic 终止程序
  p1();
  p2();
  // Result 枚举类型
  // result_syntax();
  // unwrap_fun();
  // expect_fun();
  println!("{:?}", error_pass());
  println!("{:?}", error_pass2());
}

fn p1() {
  // 展开调用栈：沿着调用栈往回走，并且清除数据（释放内存）
  // [profile.release]
  // panic = 'abort'
  // 会不清除数据，交给os来处理
}

fn p2() {
  // 设置环境变量 RUST_BACKTRACE = 1 可以使得回溯错误栈
  // panic!("xx")
}

fn result_syntax() {
  // Result来处理可解决的错误，比如打开文件失败
  // prelude 的
  // enum Result<T, E> {
  //   Ok(T),
  //   Err(E),
  // }

  let f = File::open("hello.txt");
  match f {
    Ok(file) => file,
    Err(error) => {
      println!("{:?}", error.kind());
      panic!("err{:?}", error);
    }
  };
}

fn unwrap_fun() {
  let f = File::open("hello.txt").unwrap(); // 无法自定义报错信息
}

fn expect_fun() {
  let f = File::open("hello.txt").expect("msg");
}

fn error_pass() -> Result<String, io::Error> {
  // ? 的错误类型会被隐式转换，转换为接收的类型(std::convert::From)
  let mut f = File::open("hello.txt")?;
  // // ? ==
  // let mut f = match File::open("hello.txt") {
  //   Ok(file) => file,
  //   Err(e) => return Err(e), // match 里的return 是外部函数的return。不写分号的返回才是表达式的返回值
  // };

  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn error_pass2() -> Result<String, io::Error> {
  // === error_pass
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn error_return() -> Result<(), Box<dyn Error>> {
  // Box<dyn Error> 是trait对象，可以理解为可能的错误类型
  let mut f = File::open("hello.txt")?;
  Ok({})
}


// 定义一个可能失败的场景时，用Result
// 要终止并报错的时候使用 Panic!