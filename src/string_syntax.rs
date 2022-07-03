// 来自核心语言层面：str（&str）
// 来自标准库： String
// 都是 utf-8

pub fn start() {
  p1();
  p2();
}

fn p1() {
  let mut s = String::new();
  let s2 = "xx".to_string();
  let s3 = String::from("xxxx");

  s.push_str("bar");
  s.push('c');

  let s4 = s2 + "xxx" + &s3 + "xx"; //s2发生移动 不可再用
  let s5 = format!("{}xxx{}", s4, s3); // 不会取得所有权
  println!("{},{},{},{}", s, s3, s4, s5)
}

fn p2() {
  // String 不支持[] 索引的访问方式 1.因为有的字符占用多个字节 2.索引是o(1),但是string无法确定合法字符个数
  // String 是对 Vec<u8> 的包装

  let len = String::from("草").len(); //草 占用三个字节
  println!("{}", len);

  let astring = "中文";
  // 字节
  for b in astring.bytes() {
    println!("{}", b);
  }
  // 标量值
  for c in astring.chars() {
    println!("{}", c);
  }
  // 字形簇 标准库未实现


  // string切片
  let astr = &astring[0..2]; // 会导致panick 即恐慌，因为中文占用3字符，没有按照边界个数切割会导致panick
  println!("{}", astr);

}
