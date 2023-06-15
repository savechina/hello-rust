use super::Solution;
use std::collections::HashMap;
use std::vec::Vec;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let _len = nums.len();

        if _len < 2 {
            panic!("输入参数小于2个")
        }

        // nums.iter()
        //     .enumerate()
        //     .for_each(|(i, x)| println!("{i},{x}"));

        let mut map: HashMap<i32, i32> = HashMap::new();

        // let mut i=0;
        for i in 0.._len {
            //遍历 数组值
            let n = nums[i];

            //search 差结果

            let m = target - n;

            let a = map.get_key_value(&m);

            match a {
                Some((b, c)) => {
                    println!("{:?},index:{}", b, c);
                }
                _ => {
                    print!("not match:");
                }
            }
            println!("{:?}", a);

            if map.contains_key(&m) {
                result.push(i as i32);

                let j = map[&m];

                result.push(j);

                println!("n:{}, m:{}, i:{}, j:{:?} ", n, m, i, j);
            }

            map.insert(n, i as i32);

            //查询差结果在数组中的索引
            // let a = &nums.binary_search(&m);

            // println!("n:{}, m:{}, i:{}, j:{:?} ", n, m, i, &a);

            // if a.is_ok() {
            //     result.push(i as i32);
            //     result.push(a.ok().unwrap() as i32);

            //     break;
            // } else {
            //     println!("error ,{:?}", a);
            // }
        }

        return result;
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let _len = nums.len();

        if _len < 2 {
            panic!("输入参数小于2个")
        }

        let mut map: HashMap<i32, i32> = HashMap::new();

        // let mut i=0;
        for i in 0.._len {
            //遍历 数组值
            let n = nums[i];

            //search 差结果

            let m = target - n;

            let a = map.get_key_value(&m);

            match a {
                Some((b, c)) => {
                    result.push(i as i32);

                    let j = c;

                    result.push(*j);

                    println!("n:{}, m:{}, i:{}, j:{:?} ", n, m, i, j);
                    println!("{:?},index:{}", b, c);
                }
                _ => {
                    print!("not match:");
                }
            }

            map.insert(n, i as i32);

            //查询差结果在数组中的索引
            // let a = &nums.binary_search(&m);

            // println!("n:{}, m:{}, i:{}, j:{:?} ", n, m, i, &a);

            // if a.is_ok() {
            //     result.push(i as i32);
            //     result.push(a.ok().unwrap() as i32);

            //     break;
            // } else {
            //     println!("error ,{:?}", a);
            // }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn main_test() {
        let nums = vec![3, 5, 6];
        let target = 9;

        println!("输入参数 nums：{:?}", nums);
        println!("输入参数target：{}", target);

        let r = Solution::two_sum(nums, target);

        println!("result:{:?}", r)
    }
}
