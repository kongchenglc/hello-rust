// crate
//  - binary(src/main.rs 可以有多个 binary crate,放在src/bin下的每个文件都是单独的crate)
//  - library(src/lib.rs 只能有0-1个l ibrary crate)

// lib.rs 是一个隐式的名为crate根模块。（main.rs也会）

mod m1 {
  pub mod sonM {
    pub fn a() {}
  }
  mod sonM2 {} // 默认是模块私有的,父级无法访问子的private
}

pub fn cal_mod() {
  crate::m1::sonM::a(); // 绝对路径
  m1::sonM::a(); // 相对路径
}
