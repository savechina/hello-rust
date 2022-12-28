mod my_mod;
mod my_visiable;
mod rectangle;

use crate::rectangle::Rectangle;

fn main() {
    println!("Hello, world!");

    //数字计算方法
    number_calc();

    println!("add fn :{}", add(2, 3));

    //条件表达式
    condition_sample();

    //循环表达式
    loops_sample();

    //所有权
    ownership_sample();

    //结构体作用域
    rectangle_sample();
}

/**
 * 所有权案例
 */
fn ownership_sample() {
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
}

/**
 * 数字计算方法。
 * 加、减、乘、除、余
 */
fn number_calc() {
    let sum = 5 + 10;
    // 加

    let difference = 95.5 - 4.3;
    // 减

    let product = 4 * 30;
    // 乘

    let quotient = 56.7 / 32.2;
    // 除

    let remainder = 43 % 5;
    // 求余

    println!(
        "sum: {}, diff: {}, product: {}, quotient: {}, remainder:{}",
        sum, difference, product, quotient, remainder
    );
}

/**
 *函数 求和
 */
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

///
///条件表达式
///
fn condition_sample() {
    let a = 12;
    let b;

    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }

    println!("b is {}", b);
}

/**
 * 循环表达式
 */
fn loops_sample() {
    //数组
    let a = [10, 20, 30, 40, 50];

    // for 迭代器
    for i in a.iter() {
        println!("值为 : {}", i);
    }
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
    let rect1 = Rectangle::create(30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1's area is {}", rect1.area());

    println!("rect1 is wider rect2 :{}", rect1.wider(&rect2));
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
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[ignore]
    #[test]
    fn test_bad_add() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        assert_eq!(add(1, 2), 3);
    }
}
