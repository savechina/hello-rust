mod basic;
mod leetcode;

//
//Mian 函数
//
fn main() {
    println!("Hello, world!");

    //Rust基础样例
    basic::basic_example();

    //Rust LeetCode 题目解决答案，样例代码
    leetcode::leetcode_example();

    // leet_code();
}

/**
 * 函数 求和
 */
fn add(a: i32, b: i32) -> i32 {
    return a + b;
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
