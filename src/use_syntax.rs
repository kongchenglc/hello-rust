mod a {
  pub mod b {
    pub fn c() {}
    fn d() {}
  }
}

use a::b::c; //类似符号链接, 可以用绝对也可以用相对
             //use a::b::d; //(x

pub use a::b; // use 默认是私有的，可以带pub
use a::b as d; // as 别名

pub fn start() {
  c();
  b::c();
  d::c();
}

use rand::Rng; // 用外部包 std可以直接用，不用在dependencies里写

// use std::cmp::Ordering;
// use std::io;
use std::{self, cmp::Ordering, io};
use std::io::*;
