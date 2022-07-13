// use hello_rust::cal_mod; //  如果没有循环引用，bin里的可以这样use lib里的

// package - cargo
// crate
//  - binary(src/main.rs 可以有多个 binary crate,放在src/bin下的每个文件都是单独的crate)
//  - library(src/lib.rs 只能有0-1个l ibrary crate) // 只有lib crate才能暴露函数给其他crate用，bin只能独立运行
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
mod struct_pub; // 目录结构和模块层级一致

fn call_struct() {
  let aaa = struct_pub::a::create_pub();
  // aaa.bb // (x private
}

//与struct不同，因为enum本身是个值， pub 的 enum 里面所有可能都是pub的
