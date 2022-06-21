pub fn start() {
  after();
  statement();
  println!("{}", five())
}

fn after() {
  println!("define after start!")
}

fn statement() {
  let y = {
    let x = 1;
    x + 3 //块表达式返回最后一个表达式的值， 如果加了「;」 相当于返回空 tuple -> ()
  };
  println!("{}", y)
}

fn five() -> i32 {
  5
}
