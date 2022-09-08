// 类型别名

// use std::io::Error 的应用：
// type Result<T> = Resule<T, std::io::Error>

pub fn start() {
    alias_type();
    // never_type();
    dyn_size_type();
}

fn alias_type() {
    type kilometers = i32; // 用来表示单位

    let x: i32 = 5;
    let y: kilometers = 10;
    println!("{}", x + y)
}

fn never_type() -> ! {
    loop {} // 什么都不会返回（ panic 和 loop 里的continue 也会返回 never
}

fn dyn_size_type() {
    // ---------------
    // str 是动态大小的类型，&str存储的是str的地址和str的长度
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // ---------------
    // trait 对象就是动态大小的
    trait Trait {}
    type dyn_type = Box<dyn Trait>;

    // ---------------
    // Sized 约束: 如果在编译时期一个类型的大小可以确定，它就会自动实现Sized这个trait
    // 泛型函数默认只能接受大小确定的类型（即受Sized约束，但是可以用 T: ?Sized 来取消这个约束(? 这个语法只能用于Sized上)
    fn generic<T: ?Sized>(t: &T) {}
}
