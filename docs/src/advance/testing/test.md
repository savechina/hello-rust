# 测试基础

## 开篇故事

想象你在建造一座大桥。你不会等到桥建好了才测试它是否稳固——你会在每一步都进行检查：地基是否牢固？钢筋强度够吗？混凝土配比正确吗？软件测试也是如此。测试不是最后才做的事情，而是贯穿整个开发过程的质量保障。

Rust 的测试系统就像一位严格的质检员——它在编译时就确保你的代码符合预期，让 bug 无处藏身。

---

## 本章适合谁

如果你想学习如何编写可靠的 Rust 代码，或者理解测试在 Rust 中的最佳实践，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Rust 测试的三种类型（单元、集成、文档）
2. 使用 `#[cfg(test)]` 组织测试模块
3. 使用 `assert!`、`assert_eq!`、`assert_ne!` 宏
4. 编写会 panic 的测试 (`#[should_panic]`)
5. 使用 `#[ignore]` 跳过慢测试
6. 运行特定测试和并行测试

---

## 前置要求

- [函数基础](../../basic/functions.md) - 函数定义
- [模块系统](../../basic/module.md) - 模块组织

---

## 第一个例子

最简单的测试：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

**运行测试**:
```bash
cargo test
```

**发生了什么？**

- `#[cfg(test)]` - 只在测试时编译
- `#[test]` - 标记测试函数
- `assert_eq!` - 断言相等

---

## 原理解析

### 1. 测试的三种类型

```
测试类型
├── 单元测试 (Unit Tests)
│   ├── 测试单个函数/模块
│   ├── 放在 src/ 文件中
│   └── 使用 #[cfg(test)]
├── 集成测试 (Integration Tests)
│   ├── 测试公共 API
│   ├── 放在 tests/ 目录
│   └── 像外部用户使用
└── 文档测试 (Doc Tests)
    ├── 测试文档示例
    ├── 放在 /// 注释中
    └── cargo test 自动运行
```

### 2. 单元测试组织

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
    }
}
```

### 3. Assert 宏家族

```rust
#[test]
fn test_assertions() {
    // assert! - 条件必须为 true
    assert!(true);
    assert!(2 + 2 == 4);

    // assert_eq! - 两个值相等
    assert_eq!(4, 2 + 2);
    assert_eq!("hello", "hello");

    // assert_ne! - 两个值不相等
    assert_ne!(4, 5);
    assert_ne!("hello", "world");

    // 自定义错误消息
    assert!(2 + 2 == 4, "数学出错了！");
    assert_eq!(4, 2 + 2, "加法应该工作");
}
```

### 4. 应该 Panic 的测试

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为 0");
    }
    a / b
}

#[test]
#[should_panic(expected = "除数不能为 0")]
fn test_divide_by_zero() {
    divide(10, 0);
}
```

### 5. 忽略慢测试

```rust
#[test]
fn test_fast() {
    assert_eq!(1 + 1, 2);
}

#[test]
#[ignore]
fn test_slow() {
    // 这个测试很慢，默认跳过
    std::thread::sleep(std::time::Duration::from_secs(10));
    assert!(true);
}
```

**运行被忽略的测试**:
```bash
cargo test -- --ignored
```

### 6. 测试结果类型

```rust
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("数学出错了"))
    }
}
```

### 7. 使用 nextest 批量测试

**nextest** 是 Rust 的下一代测试运行器，比 `cargo test` 更快、更强大。

**安装**:
```bash
cargo install cargo-nextest
```

**基本使用**:
```bash
# 运行所有测试
cargo nextest run

# 运行特定测试
cargo nextest run test_add

# 显示测试输出
cargo nextest run --nocapture

# 并行运行（默认使用所有 CPU 核心）
cargo nextest run --test-threads 4
```

**nextest vs cargo test 对比**:

| 特性 | cargo test | cargo nextest |
|------|-----------|---------------|
| 执行方式 | 单进程 | 每测试一进程 |
| 并行度 | 有限 | 完全并行 |
| 失败隔离 | 差（一个失败影响其他） | 好（完全隔离） |
| 重试支持 | 无 | 支持 `--retries` |
| 进度显示 | 简单 | 详细进度条 |
| 速度 | 较慢 | 快 2-5 倍 |

**高级功能**:

```bash
# 重试失败的测试
cargo nextest run --retries 2

# 只运行失败的测试
cargo nextest run --no-run  # 先记录
cargo nextest run --rerun   # 重跑失败

# 生成 JUnit 报告
cargo nextest run --message-format junit > report.xml

# 按特性过滤
cargo nextest run --features "feature1,feature2"

# 跳过特定测试
cargo nextest run --filter-expr "not test(/slow/)"
```

**在 CI/CD 中使用**:
```yaml
# GitHub Actions 示例
- name: Install nextest
  run: cargo install cargo-nextest --locked

- name: Run tests
  run: cargo nextest run --retries 2
```

**为什么选择 nextest？**
- 测试隔离：每个测试在独立进程中运行
- 快速失败：立即显示失败信息
- 更好的输出：彩色输出、进度条、详细统计
- CI 友好：原生支持重试和报告生成

---

## 常见错误

### 错误 1: 忘记 #[cfg(test)]

```rust
// ❌ 错误：测试代码会被编译到生产代码中
mod tests {
    #[test]
    fn test_something() {}
}

// ✅ 正确：只在测试时编译
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
```

### 错误 2: 测试依赖外部状态

```rust
// ❌ 错误：依赖文件系统
#[test]
fn test_read_file() {
    let content = std::fs::read_to_string("data.txt").unwrap();
    assert_eq!(content, "expected");
}

// ✅ 正确：使用临时文件或 mock
#[test]
fn test_read_file() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_data.txt");
    std::fs::write(&file_path, "expected").unwrap();
    // 测试完成后自动清理
}
```

---

## 动手练习

### 练习 1: 编写测试

为以下函数编写完整的测试：

```rust
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// TODO: 编写测试覆盖：
// - 正偶数
// - 正奇数
// - 零
// - 负偶数
// - 负奇数
```

<details>
<summary>点击查看答案</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_even() {
        assert!(is_even(2));
        assert!(is_even(100));
    }

    #[test]
    fn test_positive_odd() {
        assert!(!is_even(1));
        assert!(!is_even(99));
    }

    #[test]
    fn test_zero() {
        assert!(is_even(0));
    }

    #[test]
    fn test_negative_even() {
        assert!(is_even(-2));
        assert!(is_even(-100));
    }

    #[test]
    fn test_negative_odd() {
        assert!(!is_even(-1));
        assert!(!is_even(-99));
    }
}
```
</details>

---

## 故障排查

### Q: 如何运行单个测试？

**A**: `cargo test test_name`

### Q: 如何并行运行测试？

**A**: `cargo test -- --test-threads=4`

### Q: 如何显示测试输出？

**A**: `cargo test -- --nocapture`

---

## 小结

**核心要点**：

1. **#[cfg(test)]**: 只在测试时编译
2. **#[test]**: 标记测试函数
3. **Assert 宏**: 验证预期结果
4. **should_panic**: 测试错误处理
5. **ignore**: 跳过慢测试

---

## 术语表

| English           | 中文       |
| ----------------- | ---------- |
| Unit Test         | 单元测试   |
| Integration Test  | 集成测试   |
| Doc Test          | 文档测试   |
| Assertion         | 断言       |
| Panic             | 恐慌       |
| Test Fixture      | 测试夹具   |

---

完整示例：`src/advance/testing/test_sample.rs`

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[cfg(test)]` 的作用是什么？

2. `assert!`、`assert_eq!`、`assert_ne!` 的区别？

3. 如何测试会 panic 的函数？

<details>
<summary>点击查看答案与解析</summary>

1. 只在测试编译时包含代码
2. `assert!` = 条件为真，`assert_eq!` = 相等，`assert_ne!` = 不相等
3. 使用 `#[should_panic]` 属性

**关键理解**: 测试是代码质量的重要保障。
</details>

## 继续学习

- 下一步：[模拟测试](mock.md)
- 进阶：[测试框架](rspec.md)
- 回顾：[函数基础](../../basic/functions.md)

> 💡 **记住**：好的测试是代码最好的文档！
