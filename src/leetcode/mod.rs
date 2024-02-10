//! LeetCode 学习样例解题代码
//!
//! 问题解决rust 实例代码运行，直接使用单元测试的mian_test 运行

pub struct Solution;

//问题0001的解决
mod solution_0001;

/**
 * leetcode 样例入口
 */
pub fn leetcode_example() {
    Solution::two_sum(vec![2, 3, 5, 6], 8);

    Solution::two_sum2(vec![2, 3, 5, 6], 8);
}
