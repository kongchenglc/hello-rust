use std::ops::Deref;
/// 智能指针的结构体通常会实现这两个trait: Deref Drop
/// - Deref: 允许像引用一样使用
/// - Drop: 自定义离开作用域时
/// 常见的智能指针
/// - Box<T>: 在heap内存上分配值
/// - Rc<T>: 启用多重所有权的引用计数类型
/// - Ref<T>&RefMut<T>: 通过RefCell<T>访问，在运行时而非编译时强制借用规则
pub fn start() {
    box_point();
    reference_counting()
}

// 自定义一种智能指针: Deref trait
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// 实现 Drop trait(在prelude里)
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.data)
    }
}

fn box_point() {
    // Box 在离开作用域的时候内存和地址都会释放
    let a = Box::new(5);
    println!("{}", *a);

    // 使用自定义的智能指针类型(可以使用*来解引用)
    let b = MyBox::new(5);
    println!("{}", *b); // rust会隐式的这样调用 *(b.defef())

    // 使用自定义了Drop的
    let c = CustomSmartPointer {
        data: String::from("xxx"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("yyy"),
    };

    // 递归类型(链表)
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/// Rc<T>: 只能用于单线程,且为不可变引用,引用计数为0后才会被清除(离开作用域也会减1)
use std::rc::Rc;
// use list::{Cons,List};
fn reference_counting() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // a 链表: head -> 5 -> Nil
    // b 链表: head -> 3 -> a
    // c 链表: head -> 4 -> a

    // let a = Rc::new(Cons(5, Rc::new(Nil))); // 引用计数
    // a.clone(); // clone区别：深拷贝，复制数据

    // let b = Cons(3, Rc::clone(&a)); // 引用计数加1，不深拷贝数据
    // let c = Cons(4, Rc::clone(&a)); // 引用计数加1，不深拷贝数据
}
