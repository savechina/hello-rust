4. 如何解决函数内部定义变量如何返回出来的问题？
这是 Rust 所有权系统的核心问题。你不能返回一个指向函数内部栈上定义的局部变量的引用。 函数执行完毕后，栈上的局部变量会被清理掉，返回的引用将指向无效内存（悬垂指针

```Rust
// 这是错误的示例！悬垂指针！
fn create_value_and_return_ref<'a>() -> &'a i32 {
    let value = 42; // value 在函数栈上
    // &value // 错误！不能返回指向栈上局部变量的引用，因为它在函数返回后就无效了
}

// 这是正确的，但需要一个外部的生命周期 'a
fn get_ref_from_external_source<'a>(data: &'a i32) -> &'a i32 {
    data // data 是从外部借用进来的，它的生命周期 >= 函数返回的引用生命周期
}
```

解决方法：

要将函数内部创建的数据“返回”出来，你必须转移该数据的所有权。Rust 的移动语义（Move Semantics）使得这变得简单且安全：

* 直接返回数据 (按值返回): 函数返回类型是 T，你直接返回函数内部创建的变量。数据的所有权从函数内部转移到调用者。

```Rust
fn create_value_and_return_owned() -> i32 {
    let value = 42; // value 在函数栈上
    value // value 的所有权被移出函数
} // value 在这里不会被 drop，因为它已经被移出

fn create_string_and_return_owned() -> String {
    let text = String::from("hello"); // text 在函数栈上，但其数据在堆上
    text // text 的所有权被移出函数。堆上的数据不会被清理。
} // text 在这里不会被 drop

fn create_box_and_return_owned() -> Box<i32> {
    let boxed_value = Box::new(100); // boxed_value 在函数栈上，它指向堆上的数据
    boxed_value // boxed_value 的所有权被移出函数。堆上的数据不会被清理。
} // boxed_value 在这里不会被 drop

```

* 返回智能指针: 如果你需要共享数据，可以将内部创建的数据包装在 Rc 或 Arc 等智能指针中，并返回智能指针的副本。数据的实际所有权由智能指针管理，而你返回的是智能指针的共享引用或智能指针本身（所有权转移）。

```Rust
use std::sync::Arc;
use std::cell::RefCell;

fn create_shared_data() -> Arc<RefCell<i32>> {
    let data = RefCell::new(0); // data 在函数栈上，它包装了堆上的数据
    Arc::new(data) // Arc::new 会将 RefCell 移动到堆上，并返回 Arc 的所有权
} // data 在这里不会被 drop，因为它内部的 RefCell 已经被移到堆上并被 Arc 拥有

fn ownership_shared_sample() {
    let shared = create_shared_data(); // shared 现在拥有 Arc 的所有权
    println!("{}", shared.borrow());
}
```

```Rust
///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_ownership_shared_sample() {
        ownership_shared_sample();
        println!("print test in mdbook")
    }
}

```
