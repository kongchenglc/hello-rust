// 迭代器都是惰性的，不执行就不会消耗
// 迭代器是零开销抽象，对比手写for性能无损耗
pub fn start() {
    other_iter();
    other_next();
    map_fun();
    filter_fun();
    diy_iter()
}

fn iter_fun() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // 每次掉用next消耗v1_iter 所以得mut
    println!("{:#?}", v1_iter.next());

    // v1_iter所有权会给for，for内部会变为mut
    for val in v1_iter {
        println!("{}", val);
    }
}

fn other_iter() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter1 = v1.iter_mut(); // 迭代可变的引用
    let v1_iter = v1.into_iter(); // 迭代器会获得所有权
    println!("{:#?}", v1_iter);
}

fn other_next() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // sum会消耗迭代器
    println!("{:#?}", total);
}

fn map_fun() {
    let v1 = vec![1, 2, 3];

    // <_> 表示类型是推断出来的
    // map会产生一个新的迭代器，闭包作用于每个元素
    // collect 用于消耗迭代器，将结果收集到一个集合类型中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:#?}", v2);
}

fn filter_fun() {
    let v1 = vec![1, 2, 3];
    let flag = 1;

    // &flag 闭包捕获所在环境的变量
    let v2: Vec<_> = v1.iter().map(|x| x > &flag).collect();
    println!("{:#?}", v2);
}

fn diy_iter() {
    #[derive(Debug)]
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    //Iterator 是一个trait。可以在自己的struct自定义的实现这个trait
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let a = Counter::new();
    for item in a {
        println!("{:?}", item)
    }

    let sum: u32 = Counter::new()
        // zip 将两个迭代器拉链合并，zip返回一个()类型的迭代器
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("{:#?}", sum)
}
