use std::collections::HashMap;
// 不在 prelude 中，hashmap是同构的，所有kv类型一致。默认的hash函数可以被修改

pub fn start() {
  p1();
  p2();
  p3();
  p4();
  p5();
  p6();
}

fn p1() {
  let mut scores = HashMap::new();
  scores.insert(String::from("key1"), 10);
  scores.insert(String::from("key2"), 20);
}

fn p2() {
  let teams = vec![String::from("blue"), String::from("Yellow")];
  let intial_scores = vec![10, 50];

  // ()tuple 可以用于组件hashmap
  let a: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
}

fn p3() {
  // 所有权会移动到hashmap
  let a = String::from("xx");
  let b = String::from("xxx");

  let mut map = HashMap::new();
  // map.insert(a, b); // (x 所有权移动,后面不能再使用
  map.insert(&a, &b); // 借用
  println!("{},{}", a, b)
}

fn p4() {
  // 访问hashmap
  let mut map = HashMap::new();
  map.insert("a", 10);
  map.insert("b", 20);

  let a = "a";
  let value = map.get(&a);

  match value {
    Some(s) => println!("{}", s),
    None => println!("not exit"),
  }
}

fn p5() {
  // for
  let mut map = HashMap::new();
  map.insert("a", 10);
  map.insert("b", 20);

  for (k, v) in map {
    println!("{}:{}", k, v);
  }
}

fn p6() {
  // update hashmap

  // hashmap 大小可变
  let mut map = HashMap::new();
  map.insert("a", 10);
  map.insert("b", 20);

  // 替换现有value
  map.insert("a", 30);
  println!("{:?}", map);

  // 不存在再写入
  map.entry("a").or_insert(40);
  println!("{:?}", map);
  map.entry("c").or_insert(40);
  println!("{:?}", map);

  // 基于现有的value做修改
  let value = map.entry("c").or_insert(40);
  *value += 1;
  println!("{:?}", map);
}
