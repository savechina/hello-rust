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

trait UnObjectSafeTrait {
    fn create() -> Self; // Error: method is not object-safe
}

struct UnObjectSafeStruct {
    count: i32,
}
impl UnObjectSafeTrait for UnObjectSafeStruct {
    fn create() -> Self {
        UnObjectSafeStruct { count: 0 }
    }
}

// Error: method is not object-safe
// fn create_trait_object() -> Box<dyn UnObjectSafeTrait> {
//     Box::new(UnObjectSafeStruct::create())
// }

fn un_object_safe_sample() {
    let _instance = UnObjectSafeStruct::create();
    println!("UnObjectSafeTrait print: {:?}", _instance.count);
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

    #[test]
    fn test_un_object_safe_sample() {
        un_object_safe_sample();
    }
}
