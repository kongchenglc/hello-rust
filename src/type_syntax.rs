// 单态化：范型会在编译时单态化，范型和确定的类型执行速度一样（即确定类型
pub fn start() {
  fn_type();
  struct_type();
  enum_type();
}

fn fn_type() {
  let a = [String::from("xx")];
  let b = ['1'];
  type_fn(&a);
  type_fn(&b);

  fn type_fn<T>(list: &[T]) -> &T {
    return &list[0];
  }
}

fn struct_type() {
  #[derive(Debug)]
  struct Point<T> {
    x: T,
    y: T,
  }

  // impl<T>表示对类型T实现方法
  impl<T> Point<T> {
    // 方法的T定义在impl之后
    fn x(&self) -> &T {
      return &self.x;
    }
  }

  impl Point<i32> {
    // 如果impl没有定义范型，就需要写死类型，只有这个类型下才会有这些方法，即c++的偏特化
    fn xi32<V>(&self, a: V) -> (&i32, V) {
      // 方法自己的范型V并不互相干扰
      return (&self.x, a);
    }
  }

  let a_point = Point { x: 1, y: 2 };
  println!("{:?}", a_point);
}

fn enum_type() {
  enum Option<T> {
    Some(T),
    None,
  }

  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
}
