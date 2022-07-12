pub fn start() {}

#[cfg(test)] // 测试模块/测试函数 如果触发panic就算失败
mod test {
  use super::a_fun;

  #[test]
  fn a_test() {
    println!("111");
    // panic!("2")
  }

  #[test]
  fn a_test_2() {
    // assert 断言
    assert!(!a_fun(), "???{}", 11); // 多加参数是自定义错误消息
    assert_eq!(1, 1); // 相等？
    assert_ne!(1, 2); // 不等？
  }

  // 验证代码是否发生了panic
  #[test]
  #[should_panic(expected = "xx")] // 期待panic的报错内容
  fn throw_panic() {
    panic!("xx")
  }

  // 测试中直接使用 Result<T,E>
  #[test]
  fn it_work() -> Result<(), &'static str> {
    if 2 + 2 == 5 {
      Ok(())
    } else {
      Err("xxx")
    }
  }
}

fn a_fun() -> bool {
  1 > 2
}
