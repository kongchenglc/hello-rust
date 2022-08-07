// Rust避免将struct和enum称为对象，因为他们与impl块是分开的
// trait对象（Box<dyn Xxx>）类似其他语言中的对象，因为它在一定程度上组合了数据和行为（用于抽象某些共有行为。但是无法给trait对象添加数据
pub fn start() {
    // gui();
    // trait约束作用于范型时，rust编译器会执行单态化（编译器会对范型的每个代入的类型生成一个静态派发、穷举
    // trait对象是动态派发：无法在编译过程中确定调用的是哪种方法，会产生运行时开销

    // trait对象必须保证对象安全，对象安全：
    // 1.方法返回的类型不是Self
    // 2.方法中不包含任何范型类型参数

    state_pattern();
    state_pattern_2();
}

fn gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 20,
                height: 20,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 20,
                height: 20,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw>：trait对象
    // dyn Draw： 表示实现了Draw这个trait的类型
    // 因为大小是不确定的，所以只能用Box指针
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制一个按钮");
    }
}

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制一个选择框");
    }
}

// -----------------类似状态机的设计模式(x：不推荐)
fn state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//-----------将状态和行为 编码为类型（推荐
fn state_pattern_2() {
    let mut post = Post2::new();
    println!("{:#?}", &post);
    post.add_text("I ate a salad for lunch today");
    println!("{:#?}", &post);
    let post = post.request_review();
    println!("{:#?}", &post);
    let post = post.approve();
    println!("{:#?}", &post);
}
#[derive(Debug)]
pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}

#[derive(Debug)]
pub struct Post2 {
    content: String,
}
impl Post2 {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
