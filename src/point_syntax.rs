use std::ops::Deref;
/// 智能指针的结构体通常会实现这两个trait: Deref Drop
/// - Deref: 允许像引用一样使用
/// - Drop: 自定义离开作用域时
/// 常见的智能指针
/// - Box<T>: 在heap内存上分配值  一个所有者
/// - Rc<T>: 启用多重所有权的引用计数类型  多个所有者
/// - Ref<T>&RefMut<T>: 通过RefCell<T>访问，在运行时而非编译时强制借用规则  一个所有者
pub fn start() {
    box_point();
    reference_counting();
    RefCell_T();
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

// 内部可变性: 运行在只持有不可变数据时对数据进行修改
fn RefCell_T() {
    use std::cell::RefCell;
    // 只能用于单线程
    // RefCell<T>只能有一个所有者，内部可变性，运行时校验借用规则，如果不符合会触发一个panic。
    // （x->&x借用规则：不能同时拥有多个可变引用（防止数据竞争），可以同时拥有多个不可变引用。引用总是有效的）

    let a = RefCell::new(vec![1, 2, 3]);
    // let y = &mut a; // (x 无法可变的借用不可变的值)
    a.borrow_mut().push(4); // RefCell::new 包裹的可以用这种方式借用，来改变（内部可变性

    print!("{:#?}", a);
    print!("{:#?}", *a.borrow_mut()); // borrow_mut实现了Deref，返回RefMut<T>。是智能指针
    print!("{:#?}", *a.borrow()); // borrow实现了Deref，返回Ref<T>。是智能指针

    // RefCell 会记录有多少个活跃的 RefMut<T> 和 Ref<T>：借用加1，离开作用域减1
    // RefCell 在运行时检查借用规则，只允许一个不可变借用或者多个不可变借用。如果不满足触发panic

    // -------------
    // Rc<T> 和 RefCell<T> 结合使用，实现拥有多重所有权的可变数据
    let value = Rc::new(RefCell::new(vec![2, 2, 2]));
    let value_clone1 = Rc::clone(&value);
    let value_clone2 = Rc::clone(&value);
    value.borrow_mut().push(1);
    print!("{:?},{:?},{:?}", value, value_clone1, value_clone2);

    // -------------Weak<T>
    // Rc<T> 和 RefCell<T> 结合使用，可能出现循环引用，比如环形链表
    // 为了防止循环引用可以把 Rc<T> 换成 Weak<T>
    let yy = Rc::new(11);
    let yy_strong_ref = Rc::clone(&yy);
    let yy_weak_ref = Rc::downgrade(&yy); // 弱引用有自己计数，如果强引用个数为0，弱引用会自动断开

    println!(
        "strong:{},weak:{}",
        Rc::strong_count(&yy),
        Rc::weak_count(&yy)
    );

    // 弱引用使用前得校验值仍然存在(upgrade)
    println!("{:#?}", yy_weak_ref.upgrade());
}
