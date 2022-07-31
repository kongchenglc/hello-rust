//! crate
//!  - binary(src/main.rs 可以有多个 binary crate,放在src/bin下的每个文件都是单独的crate)
//!  - library(src/lib.rs 只能有0-1个 library crate)

//! lib.rs 是一个隐式的名为crate根模块。（main.rs也会）

pub mod base_syntax;
pub mod control_syntax;
pub mod enum_syntax;
pub mod fn_syntax;
pub mod guess_num;
pub mod owner_ship;
pub mod package_syntax;
pub mod struct_syntax;
pub mod use_syntax;
pub mod vector_syntax;
pub mod string_syntax;
pub mod hashmap_syntax;
pub mod panic_syntax;
pub mod type_syntax;
pub mod trait_syntax;
pub mod life_cycle;
pub mod test;

pub use test::start; // re-export

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
