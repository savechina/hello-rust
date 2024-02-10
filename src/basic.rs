//!
//! Rust 语言学习基础功能样例代码实现
//!
//!

///表达式
pub mod expression_sample;

/// 数据类型
pub mod datatype_sample;

///模块
pub mod module_sample;
pub mod visiable_sample;

///所有权
pub mod ownership_sample;

///泛型
pub mod generic_sample;

///多线程
pub mod threads_sample;

pub mod rectangle;

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
    ownership_sample::ownership_sample();

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

    generic_sample::add_generic_sample();

    generic_sample::summary_sample();

    //多线程 创建线程
    threads_sample::create_thread_sample();

    //多线程，线程返回结果
    threads_sample::thread_callable_sample();

    //线程屏障(Barrier)
    threads_sample::thread_barrier_sample();

    //thread local
    threads_sample::thread_local_sample();
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
