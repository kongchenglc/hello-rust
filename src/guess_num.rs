use rand::Rng;
use std::cmp::Ordering;

fn guess_num() -> String {
  println!("guess num!"); // ! 宏
  let mut guess = String::new(); //  mut 可变类型  :: 关联方法

  std::io::stdin().read_line(&mut guess).expect("read error"); // & 引用

  println!("you guess num is: {}", guess);
  return guess;
}

fn gen_secret_num() -> u32 {
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("secret num is: {}", secret_number);
  return secret_number;
}

fn cmp_num(secret_num: u32) {
  loop {
    let guess_num_string: String = guess_num();
    let guess_num: u32 = match guess_num_string.trim().parse() {
      Ok(num) => num,
      Err(err) => {
        println!("Please type a number! Err:{}", err);
        continue;
      }
    };
    match guess_num.cmp(&secret_num) {
      Ordering::Less => println!("Too small!"), // match's arm
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

pub fn start() {
  cmp_num(gen_secret_num())
}
