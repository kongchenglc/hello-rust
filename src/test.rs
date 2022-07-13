// cargo test
// 默认行为：
// 1. 并行运行所有测试
// 2. 不显示所有输出（用例成功时

// 命令行参数
// 1. 串行：cargo test -- --test-threads=1
// 2. 打印输出 cargo test -- --show-output

// 3. 用函数名称来测试
// 4. 运行多个测试：指定测试名的一部分，模块名/函数名 cargo test a_test_2
// 5. 忽略某些测试#[ignore]，运行被忽略的测试：cargo test -- --ignored

// 测试分类：单元测试、集成测试
// 单元测试：一次对一个模块进行隔离的测试、可以测试private接口
// 集成测试：在库外部，和其他外部代码一样使用你的代码、只能使用public接口

pub fn start() {}

// cfg 表示下面的条目只有指定这个配置（参数：当前是test）时才会被包含
#[cfg(test)] // 测试模块/测试函数 如果触发panic就算失败。正常不会编译，cargo test才会（集成测试不需要这个标注
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

// 集成测试：覆盖率很重要
// 需要在tests目录下创建，每个文件都是一个crate
