# 测试

Rust 内置了强大的测试框架，支持单元测试、集成测试、文档测试，以及丰富的第三方测试工具。

## 为什么 Rust 的测试很重要？

- **内置支持**：无需额外配置，`cargo test` 即可运行所有测试
- **文档测试**：代码示例即测试，确保文档与代码同步
- **Mock 支持**：Mockall 等库提供强大的模拟对象功能
- **行为驱动**：rspec 支持 BDD 风格的测试编写

## 本章节内容

| 主题 | 说明 |
|------|------|
| [测试基础](./test.md) | 单元测试、集成测试、文档测试、断言宏 |
| [模拟测试](./mock.md) | Mockall 模拟对象，测试依赖注入 |
| [RSpec 行为驱动测试](./rspec.md) | BDD 风格测试，可读性强的测试用例 |
| [Getters/Setters 派生](./getset.md) | getset 派生宏简化结构体访问器 |
| [类型别名](./typealias.md) | 类型别名简化复杂类型签名 |
| [声明宏和过程宏](./macros.md) | 宏编程基础，代码生成 |

## 快速示例：单元测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another_test() {
        panic!("这个测试应该失败");
    }
}
```

## 下一步

- 学习 [测试基础](./test.md) 掌握 Rust 测试核心
- 了解 [Mockall](./mock.md) 模拟外部依赖
- 尝试 [RSpec](./rspec.md) 行为驱动开发
