/// Printable trait 特性示例
///
trait Printable {
    fn print(&self);
}

/// Person struct 示例
struct Person {
    name: String,
    age: u32,
}

/// 实现 Printable 特性
impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}
fn traits_simple_sample() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // 使用 Printable 特性的方法
    person.print();

    println!("trait simple_sample ..... end");
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
    fn test_trait_simple_sample() {
        traits_simple_sample();
    }
}
