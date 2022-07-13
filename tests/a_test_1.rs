//只测试当前文件：cargo test --test a_test_1
use hello_rust::cal_mod;
use hello_rust::fn_syntax;

#[test]
fn a_test() {
  cal_mod();
  fn_syntax::start()
}
