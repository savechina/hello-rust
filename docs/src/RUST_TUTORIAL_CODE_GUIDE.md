# Hello Rust 教程代码使用指南

## 核心理念

**本教程的所有代码示例都来自实际可运行的项目代码**

与其他教程不同，Hello Rust 教程不使用虚构的"Hello World"示例。你学到的每一行代码都能在真实项目中找到、编译、运行。

## 为什么这样做？

### 传统教程的问题

```rust
// 传统教程：编造的例子
struct FakePerson {
    name: String,
    age: u32,
}

// 学完后：不知道如何在真实项目中使用
```

### Hello Rust 的方法

```rust
// 真实项目代码：src/basic/rectangle.rs
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 学完后：可以直接在项目中运行和扩展
```

## 如何使用本教程

### 第一步：学习环境

```bash
# 1. 克隆仓库
git clone <repo_url>
cd hello-rust

# 2. 确保可以编译
cargo build --workspace
```

### 第二步：学习章节

1. 阅读文档（`docs/src/basic/`）
2. 找到对应源码（`src/basic/`）
3. 运行示例代码
4. 修改代码，观察变化

### 第三步：动手实践

```bash
# 运行基础示例
cargo run --bin basic

# 运行高级示例
cargo run --bin advance

# 运行特定示例
cargo run --bin grpc_hello_server
```

## 代码示例类型

### 教学示例 (src/basic/)

这些是**专门设计**用于教学的代码：

- ✅ 简短（通常 <100 行）
- ✅ 聚焦单一概念
- ✅ 包含注释说明
- ✅ 有单元测试

**示例**：
```rust
// src/basic/ownership_sample.rs
fn gives_ownership() -> String {
    let some = String::from("hello");
    some  // 返回所有权
}
```

### 实战示例 (src/advance/)

这些是**真实应用**代码：

- ✅ 使用外部 crates（tokio, sqlx）
- ✅ 完整的错误处理
- ✅ 展示最佳实践

**示例**：
```rust
// src/advance/tokio_sample.rs
async fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            // 异步处理
        });
    }
}
```

### 算法示例 (src/algo/)

这些展示**算法实现**：

- ✅ 复杂度分析
- ✅ 边界情况处理
- ✅ 性能考虑

**示例**：
```rust
// src/algo/calc_pi_sample.rs
fn calculate_pi(steps: usize) -> f64 {
    // 莱布尼茨公式
    let mut pi_over_four = 0.0;
    for n in 0..steps {
        // 交错级数
    }
    pi_over_four * 4.0
}
```

### 练习代码 (crates/leetcode/)

这些是**编程挑战**：

- ✅ LeetCode 原题
- ✅ 最优解法
- ✅ 测试覆盖

**示例**：
```rust
// crates/leetcode/src/solution_0001.rs
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
```

## 安全警告

本教程包含**unsafe 代码示例**，但这些仅用于教学：

### ⚠️ 危险代码（仅学习）

```rust
// src/basic/pointer_sample.rs
let s = unsafe {
    std::str::from_utf8_unchecked(bytes)  // ⚠️ 不验证 UTF-8
};
```

**学习要点**：
1. 理解为什么这是危险的
2. 学习安全的替代方案
3. 知道何时（如果必须）使用 unsafe

### ✅ 安全代码（推荐）

```rust
let s = std::str::from_utf8(bytes)?;  // ✅ 会验证
```

## 进阶学习路径

完成基础章节后：

1. **运行所有示例**
   ```bash
   cargo run --bin basic
   cargo run --bin advance
   ```

2. **理解项目结构**
   - 阅读 `crates/awesome/` 中的框架代码
   - 理解服务生命周期

3. **尝试修改**
   - 改动代码，观察编译
   - 添加功能，运行测试

4. **贡献代码**
   - 报告文档不清
   - 添加新的示例
   - 改进错误消息

## 获取帮助

遇到问题？

1. **阅读源代码**：答案就在项目中
2. **运行测试**：`cargo test`
3. **查看错误**：Rust 编译器错误消息很详细
4. **社区支持**：[Chinese Rust User Group](https://rustcc.cn/)

---

**欢迎来到真实的 Rust 世界 - 没有虚构代码，只有真实项目！** 🚀
