// 每个引用都有自己的生命周期
// 生命周期：让引用保持有效的作用域
// 大多数情况生命周期时隐式可推断的
// 少数情况需要手动标注生命周期

use std::fmt::Display;

// 生命周期的主要目的是防止悬垂引用（类似野指针？ rust不允许null或者空值存在
pub fn start() {
  use_with_value();
  fun_param_life();
  struct_lifecycle();
}

fn use_with_value() {
  // {
  //   let r: i32;
  //   let b = r; // 不合法，rust不允许null或空。必须先初始化才能使用
  //   {
  //     let x = 2;
  //     r = &x; // (不合法，因为x的生命周期已经结束
  //   }
  //   println!("{}", r);
  // }
}

fn fun_param_life() {
  // 显示生命标注
  // &'a mut i32 // 一般用简短小写表示 'a

  // let xx = 1;
  // let zz;
  // {
  //   let yy = 2;
  //   zz = a_fn(&xx, &yy); // 不合法，yy生命周期过短
  // }
  // println!("{}", zz);

  // x,y,返回值 它们的生命周期都不能短于'a // 即'a是它们的交集，生命周期交集里比较短的那部分
  // 输入生命周期(参数)、输出生命周期(返回值)
  fn a_fn<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
      x
    } else {
      y
    }
  }
}

// 生命周期的使用
// 1.函数返回引用，需要与参数的生命周期匹配
// 2.返回引用如果没有使用参数的引用，那一定会使用函数内部的变量，但内部变量会在函数结束被销毁。所以这时只能返回值不能返回引用。
// 3.struct 中的引用
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}
// fn longest2<'a>(x: &'a str, y: & str) -> &'a str {
//   let z = String::from("zz").as_str();
//   z // 不合法，z只能在当前作用域使用，函数结束变量被销毁
// }

// 结构体生命周期
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn struct_lifecycle() {
  let novel = String::from("Some years ago...");

  let first_sentence = novel.split('.').next().expect("not found '.'");

  let i = ImportantExcerpt {
    part: first_sentence, // 引用的生命周期在范围内，没问题
  };
}

// 输入生命周期(参数)、输出生命周期(返回值)

// 编译器 帮助 生命周期省略。三个规则 用于fn和impl块
// 1. 每个引用类型参数都有自己的生命周期（用于输入生命周期 （会被同步到生命周期交集里？
// 2. 如果只有一个输入生命周期，将被赋给所有的输出生命周期（用于输出生命周期
// 3. 如果有多个输入生命周期参数，但是其中一个是 &self 或者 &mut self(即当前函数是个方法)，那么self的生命周期会被赋值给所有输出生命周期参数（用于输出生命周期

// 命中1、2，可以不写'a
fn first_world<'a>(s: &'a str) -> &'a str {
  &s[0..1]
}

// 第三条
struct ImportantExcerpt2<'a> {
  part: &'a str,
}

impl<'b> ImportantExcerpt2<'b> {
  fn level(&self) -> i32 {
    3
  }

  // 因为第一条规则，&self 和 announcement生命周期已经被同步了，self的生命周期又是已知的，返回的生命周期就可以被推断出来？
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn struct_lifecycle2() {
  let novel = String::from("Some years ago...");

  let first_sentence = novel.split('.').next().expect("not found '.'");

  let i = ImportantExcerpt2 {
    part: first_sentence, // 引用的生命周期在范围内，没问题
  };
}

// 静态生命周期 'static
// 整个程序的持续时间
// 所有字符串字面值都拥有 'static 生命周期

fn str_static() {
  let a_str: &'static str = "asd"; // 直接存储在二进制程序里（类似js的基本类型，非引用的，在栈上没在heap上
}

// 生命周期也可以看作是范型的一种
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Ammouncement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
