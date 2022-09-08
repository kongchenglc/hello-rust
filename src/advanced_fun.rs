pub fn start() {
    do_twice(add_one, 1);
    closure_and_fun();
    return_closure();
}

fn add_one(x: i32) -> i32 {
    x + 1
}
// 函数指针可以作为参数的类型去传递 param: fn()->xx
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 函数指针实现了 Fn FnMut FnOnce 这三种trait（闭包会实现的trait）。所以函数指针它可以被传给一个接受闭包的函数
// 但是例如与c函数交互的场景就只希望接受fn而不希望接受闭包（c不支持
fn closure_and_fun() {
  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> =
      list_of_numbers.iter().map(|i| i.to_string()).collect();
      // map方法的定义约束了F泛型受约束 F: FnMut(Self::Item) -> B, 表示实现了FnMut这个Trait类型的参数就可以

      // ----------------
      enum Status {
        Value(u32), // 枚举变体 作为枚举中的一项
        Stop,
      }
      let v = Status::Value(3); // Status::Value 其实是个构造函数，所以也能被传入到map函数里
      let v2 = Status::Stop;
}


// （x，返回类型需要确定大小
// fn return_closure() -> dyn Fn(i32) -> i32 {
//   |x| x + 1
// }

fn return_closure() -> Box<dyn Fn(i32) -> i32>{
    Box::new(|x| x + 1)
}