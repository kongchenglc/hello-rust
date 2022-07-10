// 每个引用都有自己的生命周期
// 生命周期：让引用保持有效的作用域
// 大多数情况生命周期时隐式可推断的
// 少数情况需要手动标注生命周期

// 生命周期的主要目的是防止悬垂引用（类似野指针？ rust不允许null或者空值存在
pub fn start() {
  use_with_value();
  fun_param_life();
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


// todo: 结构体生命周期