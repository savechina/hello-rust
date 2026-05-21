# 宏编程

Rust 的宏系统允许在编译时生成代码，是元编程的核心工具。从简单的代码简化到复杂的 DSL 构建，宏无处不在。

## 为什么学习宏？

- **减少样板代码**：自动派生 Debug、Clone、Serialize 等实现
- **领域特定语言**：构建声明式 DSL，提升代码可读性
- **编译时计算**：将运行时计算提前到编译时，提升性能
- **代码生成**：根据输入自动生成重复性代码

## 本章节内容

| 主题 | 说明 |
|------|------|
| [派生宏](./getset.md) | getset 派生宏自动生成 getters/setters |
| [声明宏和过程宏](./macros.md) | 两种宏类型的区别与使用场景 |

## 宏的类型

| 类型 | 语法 | 特点 |
|------|------|------|
| 声明宏 | `macro_rules!` | 模式匹配，适合简单代码生成 |
| 派生宏 | `#[derive(...)]` | 为结构体/枚举自动生成代码 |
| 属性宏 | `#[my_attribute]` | 修饰函数或模块 |
| 函数宏 | `my_macro!()` | 类似函数调用的宏 |

## 快速示例：声明宏

```rust
macro_rules! create_function {
    ($func_name:expr) => {
        fn $func_name() {
            println!("函数 {} 被调用了", stringify!($func_name));
        }
    };
}

create_function!(hello);

fn main() {
    hello!();
}
```

## 下一步

- 学习 [声明宏和过程宏](./macros.md) 理解宏的核心概念
- 了解 [派生宏](./getset.md) 的实际应用
