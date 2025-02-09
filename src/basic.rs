//!
//! Rust语言基础教程样例代码
//! 主要包括：
//!     expression_sample 表达式
//!
//!  
//! include:
//!    expression_sample 表达式
//!
//!
//!

///表达式
pub mod expression_sample;

/// 数据类型
pub mod datatype_sample;

///模块
pub mod module_sample;

///可见性
pub mod visiable_sample;

///所有权
pub mod ownership_sample;

///泛型
pub mod generic_sample;

///多线程
pub mod threads_sample;

pub mod rectangle;

///日志 trace
pub mod yak_shave;

///日志
pub mod logger_sample;

///指针
pub mod pointer_sample;

///cfg_if
pub mod cfg_if_sample;

use rectangle::{RectangleBorrowMut, RectangleOwner, RectangleRef};

// import mod use as alias name
use crate::basic::module_sample::supper_mod as other_mod;
use crate::basic::rectangle::Rectangle;
// Import the cfg_if macro

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

    // 模块
    module_sample::function();
    // 模块
    other_mod::function();

    //字符串
    datatype_sample::string_sample();

    //集合 HasMap
    datatype_sample::hashmap_example();

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

    datatype_sample::linkedlist_sample();

    //泛型 add 加法
    generic_sample::add_generic_sample();

    //
    generic_sample::summary_sample();

    generic_sample::point_generic_sample();

    generic_sample::const_generic_sample();

    //多线程 创建线程
    threads_sample::create_thread_sample();

    //多线程，线程返回结果
    threads_sample::thread_callable_sample();

    threads_sample::thread_atomic_sample();

    //线程屏障(Barrier)
    threads_sample::thread_barrier_sample();

    //thread local
    threads_sample::thread_local_sample();

    //thread Mutex 锁，读写锁
    threads_sample::thread_lock_sample();

    //线程只调用执行一次
    threads_sample::thread_call_once_sample();

    //使用Channel 进行线程通信
    threads_sample::thread_mpsc_channel_sample();

    //logger 打印日志记录 log ,env_logger
    logger_sample::logger_print();

    //logger 使用tracing 打印日志
    logger_sample::tracing_sample();
}

/**
 * 结构体 作用域 可见性
*/
fn rectangle_sample() {
    println!("datatype struct rectangle_sample.....start");
    let rect1 = Rectangle::create(30, 50);

    //结构体进行赋值
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle::new(30, 50);

    println!("rect1's area is {}", rect1.area());

    println!("rect1 is wider rect2 :{}", rect1.wider(&rect2));

    println!("rect3's area is {}", rect3.area());

    //矩形构造器创建一个新矩形
    let rect4 = rectangle::RectangleBuilder::new()
        .width(30)
        .height(60)
        .finalize();
    println!("rect4's width is {:?}", rect4.width);
    println!("rect4's height is {:?}", rect4.height);
    println!("rect4's area is {}", rect4.area());

    //矩形构造器创建一个新矩形，使用矩形引用 重新设置矩形的宽度和高度
    let mut rect5 = rectangle::RectangleBuilder::new()
        .width(30)
        .height(60)
        .finalize();
    println!("rect5's width is {:?}", rect5.width);

    let mut rectRef = rectangle::RectangleRef {
        width: &mut rect5.width,
        height: &mut rect5.height,
    };

    println!(
        "rectRef's width is {:?},height is {:?}",
        rectRef.width, rectRef.height
    );

    // RectangleOwner 用于创建矩形实例，并且拥有矩形实例的所有权
    let mut rect_owner = RectangleOwner::new(&mut rect5.width, &mut rect5.height);

    // Immutable borrow
    let rect_ref = RectangleRef::new(&rect_owner.width, &rect_owner.height);
    println!(
        "RectangleRef: width = {}, height = {}",
        rect_ref.width, rect_ref.height
    );

    //修改矩形的宽度和高度
    // Mutable borrow
    {
        let mut rect_borrow_mut =
            RectangleBorrowMut::new(&mut rect_owner.width, &mut rect_owner.height);

        *rect_borrow_mut.width = 40;
        *rect_borrow_mut.height = 80;
    }

    println!(
        "RectangleOwner after mutation: width = {}, height = {}",
        rect_owner.width, rect_owner.height
    );

    println!("datatype struct rectangle_sample.....end\n");
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;
    #[ignore]
    #[test]
    fn basic_test() {
        //注释下面语句，因为 threads_sample::thread_atomic_sample 多个test 方法同时调用,会出现并发同步错误；
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

    #[test]
    fn test_variable_bind() {
        expression_sample::variable_bind();
    }

    #[test]
    fn test_number_calc() {
        expression_sample::number_calc();
    }

    #[test]
    fn test_rectangle_sample() {
        rectangle_sample();
    }
}
