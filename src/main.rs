// use ferris_says::say;
// use std::io::{stdout, BufWriter};

fn guess_num() {
    println!("guess num!"); // ! 宏
    let mut guess = String::new(); //  mut 可变类型  :: 关联方法

    std::io::stdin().read_line(&mut guess).expect("read error");  // & 引用

    println!("you guess num is: {}", guess)  
}

fn main() {
    guess_num();
    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
}
