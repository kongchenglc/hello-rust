#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

// enum 变体 变体可以有不同的类型及其关联的数据量
#[derive(Debug)]
enum IpAddr2 {
  v4(u8, u8, u8, u8),
  V6(String),
}

// 类似struct impl可以给enum添加方法
impl IpAddr2 {
  fn call(&self) {
    println!("{:?}", self);
  }
}

// Option\Some\None 默认引入
// enum Option<T> {
//   Some(T),
//   None,
// }

pub fn start() {
  let four = IpAddrKind::V4;
  let ip2 = IpAddr2::v4(127, 0, 0, 1);

  println!("{:?}", &four);
  ip2.call();

  let n: Option<String> = None;

  enum_and_match();
  option_match();
  if_let();
}

// enum & match
fn enum_and_match() -> u8 {
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska,
  }

  #[derive(Debug)]
  enum Coin {
    penny,
    nickel,
    quarter(UsState),
  }
  let a = Coin::penny;
  // match 必须穷举所有结果
  match a {
    Coin::nickel => {
      println!("{:?}", a);
      return 5;
    }
    Coin::quarter(state) => {
      // 绑定了值的enum 的 模式匹配(match)
      println!("{:?}", state);
      return 25;
    }
    _ => 1, // 通配
            // _ 之后的不会被match
  }
}

fn option_match() {
  let a = Some(3);
  match a {
    None => None,
    Some(num) => Some(num + 1),
  };
}

fn if_let() {
  let a = Some(3);
  // if let 对比match，只匹配一种可能
  let b: u8 = if let Some(3) = a {
    println!("three");
    1
  } else {
    println!("others");
    2
  };
  println!("{}", b);
}
