//!
//! 所有权
//!
//!

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
