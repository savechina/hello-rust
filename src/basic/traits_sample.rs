use std::collections::HashMap;

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

/// more Trait inherit sample
// Define Trait A
trait A {
    fn method_a(&self);
}

// Define Trait B that inherits from Trait A
trait B: A {
    fn method_b(&self);
}

// Our struct
struct MyStruct;

// Implement Trait B for MyStruct
// This requires MyStruct to also implement Trait A
impl B for MyStruct {
    fn method_b(&self) {
        println!("MyStruct implements method_b");
    }
}

// ERROR! Rust tells us: "the trait `A` is not implemented for `MyStruct`"
// Even though we only explicitly wrote `impl B for MyStruct`,
// because `B: A`, MyStruct *must* also satisfy `A`.
// We need to add the `impl A for MyStruct` block:
// when `A` is empty method ,must to implement "the trait `a`"
impl A for MyStruct {
    fn method_a(&self) {
        println!("MyStruct implements method_a");
    }
}

struct MyStructB;

impl A for MyStructB {
    fn method_a(&self) {
        println!("MyStructB implements method_a");
    }
}

/// more trait inherit sample
fn multi_inherit_sample() {
    let s = MyStruct;
    s.method_a(); // Can call method from A
    s.method_b(); // Can call method from B

    // We can also treat it as an A trait object because B implies A
    let a_trait_obj: &dyn A = &s;
    a_trait_obj.method_a();

    let b_trait_obj: &dyn B = &s;
    b_trait_obj.method_a(); // Can call method_a through B trait object
    b_trait_obj.method_b();
}

/// more trait dynmaic sample
fn multi_dynmaic_sample() {
    let mut context: HashMap<String, Box<dyn A>> = HashMap::new();

    let a = MyStruct;

    let b = MyStructB;

    context.insert("A".to_string(), Box::new(a));
    context.insert("B".to_string(), Box::new(b));

    for (key, value) in &context {
        println!("Key: {}", key);
        value.method_a();
    }

    let sa = &context.get("A").unwrap();

    sa.method_a();
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

    #[test]
    fn test_multi_inherit_sample() {
        multi_inherit_sample();
    }

    #[test]
    fn test_multi_dynmaic_sample() {
        multi_dynmaic_sample();
    }
}
