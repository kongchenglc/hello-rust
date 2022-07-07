use std::fmt::Display;
use std::fs::File;
// trait 抽象的定义共享行为
// 约束: 实现trait的类型 或者 trait 必须有一个是在本地 crate 里定义的。(防止冲突

// 1.只有方法签名，没有具体实现
// 2.trait可以有多个方法
// 3.实现trait的类型必须有具体实现
pub fn start() {
  let a = Tweet {
    content: String::from("1"),
  };
  // 要调用trait上的方法trait必须在当前作用域里(use 一下)
  a.summarize();
  call_summary(&a);
  call_summary2(&a);
}

// 定义 trait
pub trait Summary {
  fn summarize(&self) -> String;
  fn summarize2(&self) -> String {
    // 默认实现，可以被重写实现
    self.summarize(); // 可以调用trait其他方法
    return String::from("默认实现");
  }
}

// 定义类型
pub struct Tweet {
  pub content: String,
}
pub struct Blog {
  pub contents: String,
}

// 实现trait
impl Summary for Tweet {
  fn summarize(&self) -> String {
    String::from("1")
  }
}
impl Summary for Blog {
  fn summarize(&self) -> String {
    String::from("2")
  }
}

pub fn call_summary(item: &impl Summary) {
  // 实现了 Summary trait 的类型表示 (:impl Summary) ： impl trait
  item.summarize();
}

pub fn call_summary2<T: Summary>(item: &T) {
  // 实现了 Summary trait 的类型表示 (T: Summary + Display) ： trait bound
  item.summarize();
}

//impl Summary 作为返回值，只能返回一种实现了Summary的类型，如果是执行时才决定具体类型是不被允许的
pub fn call_summary3<T, P>(item: &T) -> impl Summary
// where 字句 用于描述范型
where
  T: Summary + Display,
  P: Summary,
{
  // 实现了 Summary trait 的类型表示 (T: Summary + Display)
  item.summarize();
  let t = Tweet {
    content: String::from("xx"),
  };
  return t;
}

pub trait Copy: Clone {
  // Empty.
}
