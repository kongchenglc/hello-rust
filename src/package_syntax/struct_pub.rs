pub struct a {
  pub aa: String,
  bb: String,
}

impl a {
  pub fn create_pub() -> a {
    let aaa: a = a {
      aa: String::from("xx"),
      bb: String::from("yy"),
    };
    println!("{}", aaa.bb);
    return aaa;
  }}
