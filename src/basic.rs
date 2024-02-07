//!
//! Rust 语言学习基础功能样例代码实现
//!
//!
pub mod datatype_sample;
pub mod expression_sample;
pub mod module_sample;
pub mod rectangle;
pub mod visiable_sample;

// import mod use as alias name
use crate::basic::module_sample::supper_mod as other_mod;
use crate::basic::rectangle::Rectangle;

///
///Basic Example
///
///
pub fn basic_example() {
    //变量赋值与绑定
    expression_sample::variable_bind();

    //数字计算方法
    expression_sample::number_calc();

    println!("add fn :{}", expression_sample::add(2, 3));

    //位运算
    expression_sample::bit_calc();

    //条件表达式
    expression_sample::condition_sample();

    //循环表达式
    expression_sample::loops_sample();

    //所有权
    ownership_sample();

    //结构体作用域
    rectangle_sample();

    module_sample::function();

    other_mod::function();

    //集合 HasMap
    datatype_sample::collections_example();

    //数组 动态数组
    datatype_sample::vet_sample();

    //数组 静态数组
    datatype_sample::array_sample();

    //枚举
    datatype_sample::enum_sample();

    //元组
    datatype_sample::tupl_sample();

    //结构体
    datatype_sample::struct_sample();
}

/**
 * 所有权案例
 */
fn ownership_sample() {
    println!("ownership_sample.....start");

    //所有权 值引用
    let s1 = gives_ownership();

    // gives_ownership 移动它的返回值到 s1
    let s2 = String::from("hello");
    
    // s2 被声明有效
    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权
    println!("s1 {}, s3 {}", s1, s3);
    // s2 value borrowed here after move,报错
    // println!("s2 value after move. {}",s2);
    // 试一试 ^ 取消此行注释

    println!("ownership_sample.....end\n");
}

/**
 *  gives ownership
 */
fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some = String::from("hello");
    // some_string 被声明有效，进入作用域

    some // 返回some_string 被当作返回值
         //并移动出给调用的函数
}

/**
 * takes ownership and gives back by return
 */
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效
    return a_string; // a_string 被当作返回值移出函数
}

/**
 * 结构体 作用域 可见性
*/
fn rectangle_sample() {
    println!("rectangle_sample.....start");
    let rect1 = Rectangle::create(30, 50);

    //结构体进行赋值
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1's area is {}", rect1.area());

    println!("rect1 is wider rect2 :{}", rect1.wider(&rect2));

    println!("rectangle_sample.....end\n");
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn main_test() {
        basic_example();
    }

    #[test]
    fn test_add() {
        assert_eq!(expression_sample::add(1, 2), 3);
    }

    #[ignore]
    #[test]
    fn test_bad_add() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        assert_eq!(expression_sample::add(1, 2), 3);
    }
}
