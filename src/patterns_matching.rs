// 模式有两种：可辩驳的(可失败的)、不可辩驳的(不可失败的)
// 可辩驳的：if let Some(x) = a_value（if let 和 while let 接收可辩驳和不可辩驳两种模式，（带 let 也是一种定义，会转移所有权
// 不可辩驳的：能匹配任何值：let x = 5; （函数参数、let语句、for循环

// - 命名的变量(y)是可以匹配任何值的无可辩驳模式 eg: match x { Some(y) => {} }
// - 用 | 来表示两个都能匹配，1..=5 表示匹配范围（'a'..='z'

// 模式可以来解构：struct、enum、tuple
pub fn start() {
    match_syntax();
    if_let_syntax();
    while_let_syntax();
    for_syntax();
    let_syntax();
    xxx(&(3, 5));
    destruct();
    destruct_enum();
    destrust_nest();
    ignore_match(1, 2);
    match_some();
    match_guard();
    bind_at()
}

fn match_syntax() {
    let a = 1;
    match a {
        // match要求列举所有可能
        1 => println!("1"),
        _ => println!("other"), // _可以匹配任何东西，不会被绑定变量
    }
}

fn if_let_syntax() {
    let age: Result<u8, _> = "34".parse();

    // 第一个age是新定义的，第二个age是上面定义的
    if let Ok(age) = age {
        println!("age matched{}", age);
    } else if true {
        println!("age not match")
    } else if let Ok(age) = age {
        println!("??")
    }
}

fn while_let_syntax() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_syntax() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{}{}", value, index)
    }
}

fn let_syntax() {
    // let 也是一种模式匹配
    // let PATTERN = EXPRESSION;
    let a = 5;
    let (x, y, z) = (1, 2, 3);
}

// 函数参数也是模式
fn xxx(&(x, y): &(i32, i32)) {
    println!("xxx{}{}", x, y)
}

// 模式可以来解构：struct、enum、tuple
fn destruct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("{}{}", x, y);

    // 模式匹配struct
    match p {
        Point { x, y: 0 } => println!("when y === 0 match this arm"),
        Point { x: 0, y } => println!("when x === 0 match this arm"),
        Point { x, y } => println!("other get x,y"),
    }
}

fn destruct_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(0, g, b) => {
            println!("Change the color to red '0', green {}, and blue {}", g, b)
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn destrust_nest() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{}{}{}{}", feet, inches, x, y);

    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    println!("{:#?}", msg);
}

fn ignore_match(_: i32, y: i32) {
    println!("ignore param1, param2:{}", y);

    // ------- _忽略部分匹配
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // 只要匹配到Some就行，里面匹配到任意值都忽略
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // ------ _开头来命名，忽略未使用的变量
    let _x = 2; //_开头的变量
    let y = 20; //_开头的变量

    // ------
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); // （可，_不会发生绑定的操作

    if let Some(a) = s {
        println!("found a string{}", a);
    }

    // println!("{:?}", s); // （x，被借用
}

fn match_some() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x > 2 => println!("{}>2", x), //if x > 2就是match的守卫
        Some(x) => println!(" {} <2", x),
        None => (),
    }
}

fn bind_at() {
    let x = 5;

    match x {
        a @ 1..=5 => println!("one through five{}", a),
        _ => println!("something else"),
    }
}
