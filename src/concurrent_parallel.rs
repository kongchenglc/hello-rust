// 竞争状态
// 死锁

// 语言实现线程的方式：1.用os的API来创建 1:1 需要较小的运行时 2. 语言自己实现的线程：M:N 需要更大的运行时
pub fn start() {
    // create_thread();
    // thread_move();
    // channel();
    // many_producer();
    // share_memory();
    threads_lock_arc();
}

use std::thread;
use std::time::Duration;

fn create_thread() {
    // thread::spawn参数为闭包，里面的逻辑是新线程里运行的代码，返回值类型为 JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("xxx{}", i);
            thread::sleep(Duration::from_micros(1));
        }
    });

    handle.join().unwrap(); // join方法会阻止当前线程的执行，知道handle表示的线程终结。
    for i in 1..=5 {
        println!("yyy{}", i);
        thread::sleep(Duration::from_micros(1));
    }
}

fn thread_move() {
    let v = vec![1, 2, 4];
    let handle = thread::spawn(move || println!("vvv{:?}", v)); // 分线程里的借用可能活的更长， 用move就没问题了
    handle.join().unwrap();
}

use std::sync::mpsc;
fn channel() {
    // multiple producer single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.try_recv().unwrap(); // 立即返回，如果有数据达到，返回Ok包含数据，没有Err。可以循环检查
    let received = rx.recv().unwrap(); // 阻塞当前线程执行，直到有值传来
    println!("got:{}", received);
}

// channel 单所有权，在用channel转移后，原来的地方就无法使用了
fn many_producer() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let tx1 = mpsc::Sender::clone(&tx);
        let vals = vec!["hi", "ni", "hao"];
        for val in vals {
            let a_string = String::from("1") + val;
            tx1.send(a_string).unwrap();
            tx.send(val.to_owned()).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 接收端可以当作迭代器使用（阻塞
    for val in rx {
        println!("{:?}", val)
    }
}

// 并发：不要使用共享内存来通信，要用通信来共享内存（go
// 但是rust也支持共享内存：（不建议
use std::sync::Mutex;
fn share_memory() {
    // Mutex: mutual exclusion 互斥锁，同一时间只允许一个线程访问数据
    // 使用前必须尝试获得锁，使用后必须释放锁
    // Mutex<T> 也是一个智能指针
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // lock会阻塞直到获取到锁为止，可能出错所以需要unwrap
        *num = 6;
    } // 作用域走完时会自动解锁lock
    println!("{:?}", m);
}

use std::sync::Arc;
fn threads_lock_arc() {
    // 只有实现了Send trait才能在线程之间安全传递
    // Rc<T>只适合单线程，Arc<T> 即原子操作：atomic。两者api相同

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", counter.lock().unwrap()); // lock智能指针自动解引用就是值
}

// ---------- RefCell<T> and Rc<T> VS Mutex<T> and Arc<T>
// Rc::new(RefCell::new(0)) 两者类似 Arc::new(Mutex::new(0))
// Mutex<T>和RefCell<T>一样也提供了内部可变性
// RefCell<T>可能造成循环引用。 Mutex<T>可能造成死锁

fn send_sync() {
    // std::marker::Send; // 实现了Send的类型可以在线程间转移所有权（几乎所有类型都实现了，除了Rc这些
    // std::marker::Sync; // 实现了Sync的类型可以在不同线程访问 (T 是 Sync。&T 就是 Send
}
