use rayon::join;
use rayon::prelude::*; // 导入 Rayon 的 trait，这样 .par_iter() 等方法才能用
use rayon::scope;
use std::thread;
use std::time::Duration;

/// Rayon 是一个用于并行计算的库，可以显著提高程序的性能。下面是一个简单的示例，展示如何使用 Rayon 来并行处理数据。
fn rayon_sample() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 顺序处理
    println!("顺序处理:");
    for x in &data {
        print!("{} ", x * 2);
    }
    println!();

    // 并行处理 (使用 for_each)
    println!("并行处理 (for_each):");
    data.par_iter().for_each(|x| {
        // 注意：并行执行顺序是不确定的
        print!("{} ", x * 2);
    });
    println!();

    // 并行处理 (使用 map 和 collect)
    println!("并行处理 (map + collect):");
    let processed_data: Vec<_> = data
        .par_iter()
        .map(|x| x * 2) // 对每个元素进行 map 操作
        .collect(); // 将结果收集到一个新的 Vec 中
    println!("{:?}", processed_data);
}

/// 并行处理示例 ，过滤并统计数据
fn rayon_filter_sample() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 并行过滤偶数并求和
    let sum_of_even: i32 = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0) // 并行过滤出偶数
        .sum(); // 并行求和

    println!("偶数的和 (并行): {}", sum_of_even);

    // 顺序版本对比
    let sequential_sum_of_even: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("偶数的和 (顺序): {}", sequential_sum_of_even);
}

/// 使用 Rayon 进行所有权管理示例
fn rayon_ownership_sample() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "rayon".to_string(),
    ];

    // 并行地将所有字符串转换为大写
    let upper_words: Vec<String> = words
        .into_par_iter() // 转移所有权
        .map(|s| s.to_uppercase())
        .collect();

    // 注意：words 变量在这里不再可用，因为所有权已被转移

    println!("大写字符串 (并行): {:?}", upper_words);
}

fn task1() -> i32 {
    println!("Task 1 started");
    thread::sleep(Duration::from_secs(1));
    println!("Task 1 finished");
    10
}

fn task2() -> i32 {
    println!("Task 2 started");
    thread::sleep(Duration::from_secs(2));
    println!("Task 2 finished");
    20
}

/// 使用 Rayon 进行并行任务的示例
fn rayon_tasks_sample() {
    println!("Starting joined tasks...");
    // 并行执行 task1() 和 task2()
    let (result1, result2) = join(task1, task2);
    println!("Results: {}, {}", result1, result2);
}

fn process_part(data: &mut [i32]) {
    // 模拟处理这部分数据
    for x in data.iter_mut() {
        *x *= 2;
    }
    println!("Processed a part");
}

/// 使用 Rayon 进行并行任务的示例
fn rayon_scope_sample() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mid = data.len() / 2;

    println!("Original data: {:?}", data);

    // **关键修改点：**
    // 在 scope 的闭包内部，但在 s.spawn 调用之前，
    // 使用 split_at_mut 安全地将 data 分割成两个独立的可变切片。
    // 现在我们有了 part1 和 part2，它们是两个独立的可变借用。
    let (first_half, second_half) = data.split_at_mut(mid);
    scope(|s| {
        // 在同一个 scope 内生成两个任务
        s.spawn(|_| {
            // 任务 1 处理前半部分
            process_part(first_half);
        });
        s.spawn(|_| {
            // 任务 2 处理后半部分
            process_part(second_half);
        });

        // scope 会等待这两个任务都完成后才结束
        // 在 scope 结束前，part1 和 part2 都是有效的，
        // 但在 s.spawn 的闭包中，它们被安全地隔离了。
    }); // scope 结束，对 data 的借用结束

    println!("Processed data: {:?}", data);
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rayon_sample() {
        rayon_sample();
    }

    #[test]
    fn test_rayon_filter_sample() {
        rayon_filter_sample();
    }

    #[test]
    fn test_rayon_ownership_sample() {
        rayon_ownership_sample();
    }

    #[test]
    fn test_rayon_tasks_sample() {
        rayon_tasks_sample();
    }

    #[test]
    fn test_rayon_scope_sample() {
        rayon_scope_sample();
    }
}
