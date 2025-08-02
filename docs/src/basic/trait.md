# 特征

特征，Rust 定义一组行为方法，类似Java 语言中的接口。

```rust

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

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;
    #[test]
    fn test_multi_inherit_sample() {
        multi_inherit_sample();
    }
}

```
