//!
//! 所有权
//!
//!

use std::{cell::RefCell, sync::Arc};

/**
 * 所有权
 */
pub(crate) fn ownership_sample() {
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
 *  返回字符串
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

// 这是错误的示例！悬垂指针！
// fn create_value_and_return_ref<'a>() -> &'a i32 {  // 这是错误的！
//
//     let value = 42; // value 在函数栈上
//                     // &value // 错误！不能返回指向栈上局部变量的引用，因为它在函数返回后就无效了
// }

// 这是正确的，但需要一个外部的生命周期 'a
fn get_ref_from_external_source<'a>(data: &'a i32) -> &'a i32 {
    data // data 是从外部借用进来的，它的生命周期 >= 函数返回的引用生命周期
}

fn create_value_and_return_owned() -> i32 {
    let value = 42; // value 在函数栈上
    value // value 的所有权被移出函数
} // value 在这里不会被 drop，因为它已经被移出

fn create_string_and_return_owned() -> String {
    let text = String::from("hello"); // text 在函数栈上，但其数据在堆上
    text // text 的所有权被移出函数。堆上的数据不会被清理。
} // text 在这里不会被 drop

fn create_box_and_return_owned() -> Box<i32> {
    let boxed_value = Box::new(100); // boxed_value 在函数栈上，它指向堆上的数据
    boxed_value // boxed_value 的所有权被移出函数。堆上的数据不会被清理。
} // boxed_value 在这里不会被 drop

fn create_shared_data() -> Arc<RefCell<i32>> {
    let data = RefCell::new(0); // data 在函数栈上，它包装了堆上的数据
    Arc::new(data) // Arc::new 会将 RefCell 移动到堆上，并返回 Arc 的所有权
} // data 在这里不会被 drop，因为它内部的 RefCell 已经被移到堆上并被 Arc 拥有

fn ownership_variable_return_sample() {
    let value = 23;
    let result = get_ref_from_external_source(&value); // 这个函数会返回一个 Box<i32>
    println!("{}", result);

    let owned_value = create_value_and_return_owned(); // 这个函数会返回一个 i32
    println!("{}", owned_value);

    let owned_string = create_string_and_return_owned(); // 这个函数会返回一个 String
    println!("{}", owned_string);

    let owned_box = create_box_and_return_owned(); // 这个函数会返回一个 Box<i32>
    println!("{}", owned_box);
    // 这里的 boxed_value 会被 drop，因为它的所有权已经被移出函数
}

fn ownership_shared_sample() {
    let shared = create_shared_data(); // shared 现在拥有 Arc 的所有权
    println!("{}", shared.borrow());
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
    fn test_ownership_sample() {
        ownership_sample();
    }

    #[test]
    fn test_ownership_variable_return_sample() {
        ownership_variable_return_sample();
    }

    #[test]
    fn test_ownership_shared_sample() {
        ownership_shared_sample();
    }
}
