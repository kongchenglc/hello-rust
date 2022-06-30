// package - cargo
// crate
//  - binary(src/main.rs 可以有多个 binary crate,放在src/bin下的每个文件都是单独的crate)
//  - library(src/lib.rs 只能有0-1个l ibrary crate)
// module
// path

fn serve_order() {}
mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order(); // 相对路径：super上一级模块
                          // crate::serve_order(); // 绝对路径(x 在crate下才可以
  }
  fn cook_order() {}
}

// struct pub: pub的struct里面的字段默认是私有的，还需要加pub
mod struct_pub {

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
    }
  }
}

fn call_struct() {
  let aaa = struct_pub::a::create_pub();
  // aaa.bb // (x private
}

//与struct不同，因为enum本身是个值， pub 的 enum 里面所有可能都是pub的
