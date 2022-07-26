use std::thread;
use std::time::Duration;

pub fn start() {
    call_simulated_expensive_calculation()
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // println!("slef.value: {:#?}, arg:{}", self.value, arg);
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn call_simulated_expensive_calculation() {
    // 闭包：类型会自动推断，一般不需要写(闭包可以看作一个匿名函数，但是闭包可以捕获自己定义时所在的作用域的变量)
    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let a = simulated_expensive_calculation(1);

    println!("{}", expensive_closure.value(1));
    println!("{}", expensive_closure.value(2)); //不会重复调用
}

// 取得所有权 FnOnce
// 可变借用 FnMut
// 不可变借用 Fn

// move 关键字，可以强制闭包获取 它使用的环境值(变量) 的所有权
fn move_syntax() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("{:#?}", x); // （x 因为move 移动了所有权
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
