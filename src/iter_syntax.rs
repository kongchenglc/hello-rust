// 迭代器都是惰性的，不执行就不会消耗
pub fn start() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // 每次掉用next消耗v1_iter 所以得mut
    println!("{:#?}", v1_iter.next());

    // v1_iter所有权会给for，for内部会变为mut
    for val in v1_iter {
        println!("{}", val);
    }

    other_iter();
    other_next();
    map_trait();
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

fn map_trait() {
    let v1 = vec![1, 2, 3];

    // <_> 表示类型是推断出来的
    // map会产生一个新的迭代器，闭包作用于每个元素
    // collect 用于消耗迭代器，将结果收集到一个集合类型中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:#?}", v2);
}
