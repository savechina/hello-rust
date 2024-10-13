use super::Solution;
use std::collections::HashMap;
use std::vec::Vec;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let result = Some(Box::new(ListNode::new(0)));

        result
    }
}

#[cfg(test)]
mod tests {

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn solution_test() {
        let nums = vec![3, 5, 6];
        let target = 9;

        println!("输入参数 nums：{:?}", nums);
        println!("输入参数target：{}", target);

        let r = Solution::two_sum(nums, target);

        println!("result:{:?}", r)
    }
}
