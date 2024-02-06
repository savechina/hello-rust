//!
//! Basic expression sample
//!
//!

/**
 * 变量绑定。
 * 变量绑定是指将一个值赋给一个名称，这样就可以在程序的其他地方使用这个名称来引用该值了。
 * 变量绑定有两部分组成：`let`关键字和类型注解（type annotation）。
 */
pub(crate) fn variable_bind() {
    println!("variable_bind sample ..... start");
    //使用下划线忽略未使用的变量
    let _n = 5;

    //变量设置为可变的
    let mut x = 5;
    println!("The old value of x is: {}", x);

    //二次赋值
    x = 6;

    println!("The new value of x is: {}", x);

    //常量
    //常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    const MAX_POINTS: u32 = 100_000;

    println!("The const value of Max Points is :{}", MAX_POINTS);

    //变量遮蔽(shadowing)
    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let x = 5;
    // 在函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    println!("variable_bind sample ..... end \n");
}

/**
* 数字计算方法。
* 加、减、乘、除、余
*/
pub(crate) fn number_calc() {
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
 * 数值计算函数 求和
 */
pub(crate) fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

/**
 * 位运算
 *
 *  |运算符	    |说明                                               |
 *  | & 位与    |	相同位置均为1时则为1，否则为0                      |
 *  | | 位或    |	相同位置只要有1时则为1，否则为0                     |
 *  | ^ 异或    |	相同位置不相同则为1，相同则为0                      |
 *  | ! 位非    |	把位中的0和1相互取反，即0置为1，1置为0               |
 *  | << 左移   |	所有位向左移动指定位数，右位补0                     |
 *  |>> 右移    |	所有位向右移动指定位数，带符号移动（正数补0，负数补1）  |
 */
pub(crate) fn bit_calc() {
    //     运算符	说明

    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    // & 位与	相同位置均为1时则为1，否则为0
    println!("(a & b) value is {}", a & b);

    // | 位或	相同位置只要有1时则为1，否则为0
    println!("(a | b) value is {}", a | b);

    // ^ 异或	相同位置不相同则为1，相同则为0
    println!("(a ^ b) value is {}", a ^ b);

    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0、
    println!("(!b) value is {} ", !b);

    // << 左移	所有位向左移动指定位数，右位补0
    println!("(a << b) value is {}", a << b);

    // >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

///
///条件表达式
///
pub(crate) fn condition_sample() {
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
pub(crate) fn loops_sample() {
    println!("loops_sample.....start");
    //数组
    let a = [10, 20, 30, 40, 50];

    // for 迭代器
    for i in a.iter() {
        println!("值为 : {}", i);
    }

    //vec 迭代器
    let v = Vec::from([3, 2, 4]);

    for (i, n) in v.iter().enumerate() {
        println!("索引：{},值为 : {}", i, n);
    }

    println!("loops_sample.....end\n");
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
    fn test_variable_bind() {
        variable_bind();
    }

    #[test]
    fn test_number_calc() {
        let sum = 5 + 10;
        // 加法测试。注意这个断言会导致测试失败。
        assert_eq!(sum, 15);
    }

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
