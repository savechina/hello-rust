//!
//!
//! Basic 泛型)Generic and  特征 Trait Sample
//!
//！

/**
 * add i8
 */
fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

/**
 * 加法 add ，两个数类型为T 的 A ,B 相加
 * T: std::ops::Add<Output = T>
 *
 */
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/**
 * 泛型 加法 样例
 */
pub(crate) fn add_generic_sample() {
    println!("generic add_generic_sample ..... start");
    //add plain sample method
    println!("plain add i8: {}", add_i8(2i8, 3i8));
    println!("plain add i32: {}", add_i32(20, 30));
    println!("plain add f64: {}", add_f64(1.23, 1.23));

    println!("generic add i8: {}", add(2i8, 3i8));
    println!("generic add i32: {}", add(20, 30));
    println!("generic add f64: {}", add(1.23, 1.23));

    println!("generic add_generic_sample ..... end\n");
}

use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

/**
 *  Point 结构体泛型
 */
pub(crate) fn point_generic_sample() {
    println!("generic point_generic_sample ..... start");
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };

    println!("p = {:?},p2={:?}", p1, p2);

    println!("p1=(p.x = {},p.y = {})", p1.x, p1.y);

    println!("generic point_generic_sample ..... end\n");
}

/**
 * const generic expression
 */
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

/**
 * const 泛型样例
 */
pub(crate) fn const_generic_sample() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

/**
 * 定义Summary trait 特征
 *
 */
pub trait Summary {
    fn summarize(&self) -> String;
}

/**
 * Post 结构体
 */
pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

/**
 * Post 实现 Summary 特征接口
 */
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

/**
 * Twitter 结构体
 */
pub struct Twitter {
    pub username: String,
    pub content: String,
}
/**
 *
 * Twitter 实现 Summary 特征接口
 *
 */
impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

/**
 * summary_sample
 */
pub(crate) fn summary_sample() {
    println!("trait summary_sample ..... start");

    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };

    let weibo = Twitter {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("Post Summary is {}", post.summarize());
    println!("Twitter summary is {}", weibo.summarize());

    println!("trait summary_sample ..... end\n");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_generic_test() {
        add_generic_sample();
    }

    #[test]
    fn point_generic_test() {
        point_generic_sample();
    }

    #[test]
    fn const_generic_test() {
        const_generic_sample();
    }
}
