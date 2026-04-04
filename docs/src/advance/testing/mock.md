# Mock 模拟测试

## 开篇故事

想象你要测试一个依赖数据库的服务。传统方式是：连接真实数据库 → 插入测试数据 → 测试 → 清理数据。Mock 就像是假数据库——它模拟数据库的行为，但不需要真实连接。mockall 库帮你轻松创建这些"假"对象。

---

## 本章适合谁

如果你需要编写单元测试（测试依赖外部服务、数据库、API），本章适合你。Mock 是单元测试的关键技术。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Mock 测试概念
2. 使用 mockall 创建 Mock 对象
3. 模拟 trait 实现
4. 设置期望和返回值
5. 验证方法调用

---

## 前置要求

- [测试基础](test.md) - 测试基础
- [特征](../basic/trait.md) - trait 基础
- [Arc 智能指针](../basic/pointer.md) - Arc 基础

---

## 第一个例子

最简单的 Mock 使用：

```rust
use mockall::automock;
use std::sync::Arc;

// 定义 trait
#[automock]
trait HmsMonitorService {
    fn monitor(&self) -> bool;
}

// 使用 trait 的结构体
#[derive(Clone)]
pub struct MonitorMessageConsumerListener {
    monitor_service: Arc<dyn HmsMonitorService>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor() {
        // 创建 Mock 对象
        let mut mock = MockHmsMonitorService::new();
        
        // 设置期望
        mock.expect_monitor()
            .returning(|| true);
        
        let listener = MonitorMessageConsumerListener {
            monitor_service: Arc::new(mock),
        };
        
        // 测试
        assert!(listener.monitor_service.monitor());
    }
}
```

**完整示例**: [mock_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/mock_sample.rs)

---

## 原理解析

### mockall 特性

**mockall 是 Mock 测试库**：

- ✅ 自动生成 Mock
- ✅ 支持 trait
- ✅ 期望验证
- ✅ 返回值设置

### 使用 automock

**使用 #[automock] 属性**：

```rust
use mockall::automock;

#[automock]
trait Database {
    fn connect(&self, url: &str) -> bool;
    fn query(&self, sql: &str) -> Vec<String>;
}
```

**生成的 Mock 类型**：
- `MockDatabase`: Mock 实现
- `expect_connect()`: 设置期望
- `expect_query()`: 设置期望

### 创建 Mock 对象

**使用 new()**：

```rust
let mut mock = MockDatabase::new();
```

### 设置期望

**使用 expect_*() 方法**：

```rust
mock.expect_connect()
    .with(eq("postgres://localhost"))  // 参数匹配
    .returning(|_| true);               // 返回值

mock.expect_query()
    .with(eq("SELECT *"))
    .returning(|_| vec!["row1".to_string()]);
```

### 验证调用

**使用 times()**：

```rust
mock.expect_monitor()
    .times(1)      // 期望调用 1 次
    .returning(|| true);
```

**验证调用顺序**：

```rust
let ctx = Mock::new_context();
mock.expect_connect().returning(|_| true);
mock.expect_query().returning(|_| vec![]);

// 按顺序调用
mock.connect("url");
mock.query("SELECT");
```

### 异步 Mock

**使用 #[async_trait]**：

```rust
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
trait AsyncService {
    async fn process(&self) -> bool;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_async() {
        let mut mock = MockAsyncService::new();
        mock.expect_process()
            .returning(|| async { true }.boxed());
        
        assert!(mock.process().await);
    }
}
```

---

## 常见错误

### 错误 1: 忘记设置返回值

```rust
mock.expect_monitor();  // ❌ 没有设置返回值
mock.monitor();         // ❌ 会 panic
```

**错误信息**:
```
MockHmsMonitorService::monitor: No matching expectation found
```

**修复方法**:
```rust
mock.expect_monitor()
    .returning(|| true);  // ✅ 设置返回值
```

### 错误 2: 参数不匹配

```rust
mock.expect_connect()
    .with(eq("postgres://localhost"));

mock.connect("mysql://localhost");  // ❌ 参数不匹配
```

**错误信息**:
```
MockDatabase::connect: No matching expectation found
```

**修复方法**:
```rust
mock.connect("postgres://localhost");  // ✅ 匹配期望
```

### 错误 3: 调用次数不匹配

```rust
mock.expect_monitor()
    .times(1);  // 期望 1 次

mock.monitor();
mock.monitor();  // ❌ 调用了 2 次
```

**错误信息**:
```
MockHmsMonitorService::monitor: Expectation called too many times
```

**修复方法**:
```rust
mock.expect_monitor()
    .times(2);  // ✅ 期望 2 次
```

---

## 动手练习

### 练习 1: 创建简单 Mock

```rust
use mockall::automock;

#[automock]
trait Calculator {
    fn add(&self, a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        // TODO: 创建 Mock
        // TODO: 设置期望
        // TODO: 验证结果
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut mock = MockCalculator::new();
mock.expect_add()
    .returning(|a, b| a + b);

assert_eq!(mock.add(2, 3), 5);
```
</details>

### 练习 2: 验证调用次数

```rust
#[automock]
trait Logger {
    fn log(&self, msg: &str);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_log_called() {
        // TODO: 创建 Mock
        // TODO: 期望调用 2 次
        // TODO: 调用并验证
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut mock = MockLogger::new();
mock.expect_log()
    .times(2)
    .returning(|_| ());

mock.log("msg1");
mock.log("msg2");
```
</details>

### 练习 3: 参数匹配

```rust
#[automock]
trait UserService {
    fn get_user(&self, id: u32) -> String;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_user() {
        // TODO: 创建 Mock
        // TODO: 设置不同 ID 的返回值
        // TODO: 验证结果
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
let mut mock = MockUserService::new();
mock.expect_get_user()
    .with(eq(1))
    .returning(|_| "Alice".to_string());
mock.expect_get_user()
    .with(eq(2))
    .returning(|_| "Bob".to_string());

assert_eq!(mock.get_user(1), "Alice");
assert_eq!(mock.get_user(2), "Bob");
```
</details>

---

## 故障排查 (FAQ)

### Q: Mock 和真实实现有什么区别？

**A**: 
- **真实实现**: 实际执行业务逻辑
- **Mock**: 模拟行为，用于测试
- **用途**: Mock 用于单元测试，隔离依赖

### Q: 什么时候使用 Mock？

**A**: 
- 测试依赖外部服务（数据库、API）
- 测试边界情况（错误、超时）
- 加速测试（避免真实 IO）

### Q: Mock 会影响性能吗？

**A**: 
- Mock 本身性能开销很小
- 主要用于测试，不影响生产性能
- 测试速度通常更快（无真实 IO）

---

## 知识扩展

### 匹配器

```rust
use mockall::predicate::*;

mock.expect_query()
    .with(eq("SELECT *"))      // 精确匹配
    .returning(|_| vec![]);

mock.expect_query()
    .with(str::starts_with("SELECT"))  // 前缀匹配
    .returning(|_| vec![]);
```

### 返回 Result

```rust
mock.expect_connect()
    .returning(|_| Ok(()));

mock.expect_connect()
    .returning(|_| Err("Connection failed"));
```

### 多次调用不同返回值

```rust
mock.expect_monitor()
    .returning_st(|| {
        static mut CALLED: u32 = 0;
        unsafe {
            CALLED += 1;
            CALLED <= 2  // 前 2 次返回 true，之后 false
        }
    });
```

---

## 小结

**核心要点**：

1. **mockall**: 自动生成 Mock
2. **#[automock]**: 为 trait 生成 Mock
3. **expect_*()**: 设置期望
4. **returning()**: 设置返回值
5. **times()**: 验证调用次数

**关键术语**：

- **Mock**: 模拟对象
- **Expectation**: 期望
- **Predicate**: 谓词匹配
- **Stub**: 桩实现

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Mock | 模拟对象 |
| Expectation | 期望 |
| Predicate | 谓词 |
| Stub | 桩 |
| Trait | 特征 |

---

## 知识检查

**快速测验**（答案在下方）：

1. `#[automock]` 属性做了什么？

2. 如何设置 Mock 的返回值？

3. Mock 和真实实现的区别？

<details>
<summary>点击查看答案与解析</summary>

1. 自动生成 trait 的 Mock 实现（`MockTraitName`）
2. `mock.expect_method().returning(|args| value)`
3. Mock 是测试用的假实现，可控行为；真实实现是生产用的

**关键理解**: Mock 让你隔离测试，不依赖外部系统。
</details>

## 继续学习

**前一章**: [Cow 类型](cow.md)  
**下一章**: [测试框架](rspec.md)

**相关章节**:
- [Cow 类型](cow.md)
- [测试框架](rspec.md)
- [特征](../basic/trait.md)

**返回**: [高级进阶](advance-overview.md)

---

**完整示例**: [mock_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/mock_sample.rs)
