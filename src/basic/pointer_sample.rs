use std::{
    slice::{self},
    str::{self},
};

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();

    (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { str::from_utf8_unchecked(slice::from_raw_parts(pointer as *const u8, length)) }
}

/// 获取字符串的内存地址和长度
fn get_str_raw_sample() {

    // 获取字符串的内存地址和长度
    let (pointer, length) = get_memory_location();

    // 读取内存地址中的字符串
    let message = get_str_at_location(pointer, length);

    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
}

/// 原始指针示例
fn raw_pointer_sample() {
    // explicit cast
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // implicit coercion
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    println!("{:?}", i);
    println!("{:?}", m);

    println!("{:?}", p_imm);
    println!("{:?}", p_mut);

    // 通过原始指针读取数据
    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;

        println!("{}", ref_imm);
        println!("{}", ref_mut);
    }
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
    fn test_raw_str() {
        get_str_raw_sample();
    }

    #[test]
    fn test_raw_pointer() {
        raw_pointer_sample();
    }
}
