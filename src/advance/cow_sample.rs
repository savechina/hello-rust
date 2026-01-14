use std::borrow::Cow;

fn filter_profanity(input: &str) -> Cow<str> {
    if input.contains("badword") {
        // 情况 A：需要修改，克隆数据并返回 Owned 变体
        let filtered = input.replace("badword", "****");
        Cow::Owned(filtered)
    } else {
        // 情况 B：无需修改，直接返回原始借用（零分配）
        Cow::Borrowed(input)
    }
}

fn cow_reader_sample() {
    let s1 = "Hello, world!";
    let res1 = filter_profanity(s1); // 此时是 Borrowed，没有内存分配

    let s2 = "This is a badword!";
    let res2 = filter_profanity(s2); // 此时是 Owned，发生了 String 分配

    println!(
        "Res 1: {} (is_owned: {})",
        res1,
        matches!(res1, Cow::Owned(_))
    );
    println!(
        "Res 2: {} (is_owned: {})",
        res2,
        matches!(res2, Cow::Owned(_))
    );
}

fn cow_writer_sample() {
    let mut cow: Cow<str> = Cow::Borrowed("original");

    // 第一次调用 to_mut()：由于是 Borrowed，会触发克隆并转换为 Owned
    cow.to_mut().make_ascii_uppercase();

    // 第二次调用 to_mut()：由于已经是 Owned，直接返回引用，不再次克隆
    cow.to_mut().push_str("!!!");

    println!("{}", cow); // 输出: ORIGINAL!!!
}

struct Page {
    id: u64,
    data: Vec<u8>,
}

fn process_page_data<'a>(page_data: &'a [u8], is_writable: bool) -> Cow<'a, [u8]> {
    let mut cow = Cow::Borrowed(page_data);

    if is_writable {
        // to_mut() 会检查。如果是 Borrowed，则执行 clone() 变成 Owned
        // 如果已经是 Owned，则直接返回引用
        let mutable_data = cow.to_mut();
        mutable_data[0] = 0xFF; // 修改标记位
    }

    cow
}

fn cow_write_page_sample() {
    let disk_data = vec![0u8; 4096]; // 模拟 mmap 映射的磁盘数据

    // 只读场景：完全不分配内存，直接引用 disk_data
    let read_only = process_page_data(&disk_data, false);

    println!("readable before : {:?}", read_only[0]);

    // 写入场景：在 to_mut() 被调用时发生一次 4KB 的拷贝
    let writable = process_page_data(&disk_data, true);

    println!("writable after: {:?}", writable[0]);
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
    fn test_cow_read_sample() {
        cow_reader_sample();

        cow_writer_sample();
    }

    #[test]
    fn test_cow_writer_sample() {
        cow_writer_sample();
    }

    #[test]
    fn test_cow_write_page_sample() {
        cow_write_page_sample();
    }
}
