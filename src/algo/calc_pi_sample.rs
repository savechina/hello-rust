use bigdecimal::BigDecimal;
use num_bigfloat::BigFloat;
use rayon::prelude::*;

/// 使用莱布尼茨公式计算 PI
/// - Parameter steps: 迭代次数
fn calculate_pi(steps: usize) -> f64 {
    let mut pi_over_four: f64 = 0.0;

    for n in 0..steps {
        // 分母为 1, 3, 5, 7...
        let denominator = (2 * n + 1) as f64;
        // 符号交替发生变化
        if n % 2 == 0 {
            pi_over_four += 1.0 / denominator;
        } else {
            pi_over_four -= 1.0 / denominator;
        }
    }

    pi_over_four * 4.0
}

pub fn calculate_pi_sample() {
    // 注意：100亿次迭代在 Rust 中运行很快，但仍需一些时间
    let iterations = 10_000_000_000;
    let result = calculate_pi(iterations);

    println!("迭代 {} 次的结果: {}", iterations, result);
    println!("系统标准 PI 值: {}", std::f64::consts::PI);
}

///使用函数方式计算Pi值
/// - Parameter steps: 迭代次数
pub fn calculate_pi_functional(steps: usize) -> f64 {
    (0..steps)
        .map(|n| {
            let val = 1.0 / (2 * n + 1) as f64;
            if n % 2 == 0 { val } else { -val }
        })
        .sum::<f64>()
        * 4.0
}

///使用Rayon 提供的Parallel Iterator,并行计算PI 值
/// - Parameter steps: 迭代次数
pub fn calculate_pi_parallel(steps: usize) -> f64 {
    // 使用 into_par_iter() 开启多线程并行计算
    let pi_over_four: f64 = (0..steps)
        .into_par_iter()
        .map(|n| {
            let val = 1.0 / ((2 * n) as f64 + 1.0);
            if n % 2 == 0 { val } else { -val }
        })
        .sum(); // Rayon 会自动在多线程间进行归约累加

    pi_over_four * 4.0
}

pub fn calculate_pi_bbp(steps: usize) -> f64 {
    (0..steps)
        .into_par_iter()
        .map(|k| {
            let k_f = k as f64;
            let p16 = 16.0f64.powf(-k_f);
            p16 * (4.0 / (8.0 * k_f + 1.0)
                - 2.0 / (8.0 * k_f + 4.0)
                - 1.0 / (8.0 * k_f + 5.0)
                - 1.0 / (8.0 * k_f + 6.0))
        })
        .sum()
}

pub fn calculate_pi_bbp_bigfloat(steps: usize) -> BigFloat {
    (0..steps)
        .into_par_iter()
        .map(|k| {
            let k_bf = BigFloat::from(k as u64);
            let eight = BigFloat::from(8.0);
            let sixteen = BigFloat::from(16.0);

            // 计算 16^-k
            let p16 = BigFloat::from(1.0) / sixteen.pow(&k_bf);

            // BBP 公式核心部分
            let term = BigFloat::from(4.0) / (eight * k_bf + BigFloat::from(1.0))
                - BigFloat::from(2.0) / (eight * k_bf + BigFloat::from(4.0))
                - BigFloat::from(1.0) / (eight * k_bf + BigFloat::from(5.0))
                - BigFloat::from(1.0) / (eight * k_bf + BigFloat::from(6.0));

            p16 * term
        })
        .sum()
}

pub fn calculate_pi_bbp_bigdecimal(steps: usize) -> BigDecimal {
    // 设置精度：目标小数位 + 一些额外守位（guard digits）避免累积误差
    let prec = steps as u64 + 32;

    let zero = BigDecimal::from(0u64);
    let one = BigDecimal::from(1u64);
    let two = BigDecimal::from(2u64);
    let four = BigDecimal::from(4u64);
    let five = BigDecimal::from(5u64);
    let six = BigDecimal::from(6u64);
    let eight = BigDecimal::from(8u64);
    let sixteen = BigDecimal::from(16u64);

    // 并行计算每项
    let sum: BigDecimal = (0..steps)
        .into_par_iter()
        .map(|k| {
            let k_bd = BigDecimal::from(k as u64);

            // 计算 1 / 16^k（从 k=0 开始迭代乘法，避免 pow）
            // let mut p16 = one.clone();
            // for _ in 0..k {
            //     p16 = p16 / &sixteen;
            // }

            // 计算 1 / 16^k
            let p16 = one.clone() / sixteen.powi(k as i64);

            // BBP 项：4/(8k+1) - 2/(8k+4) - 1/(8k+5) - 1/(8k+6)
            let term = four.clone() / (&eight * &k_bd + &one)
                - two.clone() / (&eight * &k_bd + &four)
                - one.clone() / (&eight * &k_bd + &five)
                - one.clone() / (&eight * &k_bd + &six);

            p16 * term
        })
        .sum();

    // 最终调整到目标精度
    sum.with_prec(prec)
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {

    use axum::routing::on;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_calculate_pi_sample() {
        calculate_pi_sample()
    }

    #[test]
    fn test_calculate_pi_functional() {
        let iterations = 10_000_000_000;
        let result = calculate_pi_functional(iterations);

        println!("迭代 {} 次:", iterations);
        println!("计算结果 PI 值: {}", result);
        println!("系统标准 PI 值: {}", std::f64::consts::PI);
    }

    #[test]
    fn test_calculate_pi_parallel() {
        let iterations = 10_000_000_000;
        let result = calculate_pi_parallel(iterations);

        println!("迭代 {} 次:", iterations);
        println!("计算结果 PI 值: {}", result);
        println!("系统标准 PI 值: {}", std::f64::consts::PI);
    }

    #[test]
    fn test_calculate_pi_bbp() {
        let iterations = 11;
        let result = calculate_pi_bbp(iterations);

        println!("迭代 {} 次:", iterations);
        println!("计算结果 PI 值: {}", result);
        println!("系统标准 PI 值: {}", std::f64::consts::PI);
    }

    #[test]
    fn test_calculate_pi_bbp_bigfloat() {
        let iterations = 30;
        let result = calculate_pi_bbp_bigfloat(iterations);

        println!("迭代 {} 次:", iterations);
        println!("计算结果 PI 值: {}", result);
        println!("系统标准 PI 值: {}", num_bigfloat::PI);
    }

    #[test]
    fn test_calculate_pi_bbp_bigdecimal() {
        let steps = 1024;
        let result = calculate_pi_bbp_bigdecimal(steps);

        println!("迭代 {} 次:", steps);
        println!("计算结果 PI 值: {}", result);
        println!("系统标准 PI 值: {}", num_bigfloat::PI);
    }
}
