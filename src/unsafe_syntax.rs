// unsafe rust 里可以执行的四个动作：
// 1.解引用原始引用
// 2.调用unsafe函数或者方法
// 3.访问或者修改可变的静态变量
// 4.实现unsafe trait

use core::slice;

pub fn start() {
    raw_point();

    unsafe {
        unsafe_function_method();
    }

    let str = &mut [1, 2, 3, 4, 5, 6][0..=5];
    println!("{:?}", split_at_mut(str, 3));

    call_abi();
    add_to_count(1);
}
// ----------------
// 1.解引用原始引用
fn raw_point() {
    // 原始指针： 可变的 *mut T ；不可变的 *const T，解引用后不能直接赋值。
    // 这里的*不是解引用符号，是类型名的一部分

    // 原始指针特性：
    // 1. 允许同时具有不可变和可变指针、多个指向同一位置的可变指针。忽略借用规则
    // 2. 无法保证能指向合理的内存
    // 3. 允许为null
    // 4. 不实现任何自动清理

    let mut num = 5;
    let r1 = &num as *const i32; //可以在unsafe块之外创建原始指针，但是只能在unsafe里解引用
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        // println!("r: {}", *r);
    }
}

// ----------------
// 2.调用unsafe函数或者方法
// 在定义前加了unsafe的函数或者方法
unsafe fn unsafe_function_method() {
    println!("unsafe")
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// FFI “C”定义abi调用方式 ，这里的代码都是unsafe的
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_abi() {
    unsafe {
        println!("abs:{}", abs(-3));
    }
}

// 从其他语言调用rust函数， #[no_mangle] 表示避免编译时改变函数名称。这里不需要unsafe
#[no_mangle]
extern "C" fn call_from_c() {
    println!("call a fun from C")
}

// ----------------------
// 3.访问或者修改可变的静态变量
// rust 的全局变量叫静态变量
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; //多线程会出现数据竞争
    }
}

// ---------------------
// 4. 实现不安全trait
unsafe trait Foo {}
unsafe impl Foo for i32 {}
