pub fn start() {
  if_syntax();
  match_syntax();
  loop_syntax();
  while_syntax();
  for_syntax();
}

fn if_syntax() {
  let a = if 3 < 5 {
    println!("true");
    1
  } else if 2 == 2 {
    println!("false");
    2
  } else {
    3
  };
  println!("{}", a)
}

fn match_syntax() {
  match 3 < 5 {
    true => {
      println!("match")
    }
    false => {
      println!("not match")
    }
  }
}

fn loop_syntax() {
  let mut x = 0;
  let result = loop {
    x += 1;
    if x == 5 {
      break 100 + x;
    }
  };
  println!("{}", result)
}

fn while_syntax() {
  let mut x = 3;
  while x != 0 {
    println!("{}", x);
    x -= 1
  }
}

fn for_syntax() {
  let a = [1, 2, 3, 4, 5];
  for item in a.iter() {
    println!("{}", item) //item: &32 类似指针
  }

  for num in (1..4).rev() {
    // Range 不包含结束
    println!("{}", num)
  }
}
