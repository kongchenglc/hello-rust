use std::fmt::Display;
use std::fs::File;
// trait 抽象的定义共享行为
// 约束: 实现trait的类型 或者 trait 必须有一个是在本地 crate 里定义的。(防止冲突

// 1.只有方法签名，没有具体实现
// 2.trait可以有多个方法
// 3.实现trait的类型必须有具体实现
pub fn start() {
    let a = Tweet {
        content: String::from("1"),
    };
    // 要调用trait上的方法trait必须在当前作用域里(use 一下)
    a.summarize();
    call_summary(&a);
    call_summary2(&a);
    ops_overload();
    ops_overload_t_type();
    fully_qualified_syntax()
}

// 定义 trait
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String {
        // 默认实现，可以被重写实现
        self.summarize(); // 可以调用trait其他方法
        return String::from("默认实现");
    }
}

// 定义类型
pub struct Tweet {
    pub content: String,
}
pub struct Blog {
    pub contents: String,
}

// 实现trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from("1")
    }
}
impl Summary for Blog {
    fn summarize(&self) -> String {
        String::from("2")
    }
}

pub fn call_summary(item: &impl Summary) {
    // 实现了 Summary trait 的类型表示 (:impl Summary) ： impl trait
    item.summarize();
}

pub fn call_summary2<T: Summary>(item: &T) {
    // 实现了 Summary trait 的类型表示 (T: Summary + Display) ： trait bound
    item.summarize();
}

//impl Summary 作为返回值，只能返回一种实现了Summary的类型，如果是执行时才决定具体类型是不被允许的
pub fn call_summary3<T, P>(item: &T) -> impl Summary
// where 字句 用于描述范型
where
    T: Summary + Display,
    P: Summary,
{
    // 实现了 Summary trait 的类型表示 (T: Summary + Display)
    item.summarize();
    let t = Tweet {
        content: String::from("xx"),
    };
    return t;
}

pub trait Copy: Clone {
    // Empty.
}

// -------高级trait
pub trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

// 泛型和关联类型
// 泛型可以传不同的类型，来多次实现同一个trait到xx上
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}
impl Iterator2<u32> for Counter {
    // 多次实现同一个trait
    fn next(&mut self) -> Option<u32> {
        None
    }
}
// 关联类型 无法实现多次
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        None
    }
}

//------默认泛型参数和运算符重载
fn ops_overload() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    println!("{:#?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
}

fn ops_overload_t_type() {
    #[derive(Debug)]
    struct Millimeters(u32);
    #[derive(Debug)]
    struct Meters(u32);

    use std::ops::Add;
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    let a: Millimeters = Millimeters(1);
    let b: Meters = Meters(1);
    println!("{:?}", a + b)
}

// 完全限定语法：在struct自身方法与实现了的trait的方法重名时，有可能没法区分要调用哪个，用于指定
fn fully_qualified_syntax() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

fn supertrait_syntax() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        // OutlinePrint 这个trait 依赖fmt::Display 这个trait
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

// new type 绕过 孤儿规则：只有当triat或者类型定义在本地包中时，才能为该类型实现这个trait
fn new_type() {
    // 利用 tuple struct 创建一个新的类型

    use std::fmt;

    // Vec时外部类型，不能直接加fmt::Display这个外部trait，但是包一层就可以了
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
