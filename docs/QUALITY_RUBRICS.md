# 质量审核标准 (Quality Rubrics)

**版本**: 1.0  
**创建日期**: 2026-04-04  
**适用范围**: 所有文档章节、代码示例、项目实战

---

## 一、库选择标准 (Library Selection Criteria)

用于 SC-010 "延伸阅读" 章节的库评估。

### 评估维度

#### 1. 维护状态 (Maintenance) - 权重 40%

| 指标             | ✅ 优秀 (>90 分) | 🟡 良好 (60-89 分) | ❌ 警告 (<60 分) |
| ---------------- | --------------- | ----------------- | --------------- |
| **最后更新**     | <1 年           | <2 年             | >2 年            |
| **未解决问题**   | <50 个          | <200 个           | >200 个          |
| **响应时间**     | <1 周           | <1 个月           | >1 个月          |
| **发布版本频率** | 每年 >4 次       | 每年 >2 次         | 每年 <2 次       |

**计算公式**:
```
维护得分 = (更新时间得分 × 0.4) + (问题数得分 × 0.2) + (响应时间得分 × 0.3) + (发布频率得分 × 0.1)
```

#### 2. 代码质量 (Quality) - 权重 30%

| 指标            | ✅ 优秀         | 🟡 良好       | ❌ 警告     |
| --------------- | --------------- | ------------- | ----------- |
| **GitHub Stars** | >500            | >100          | <100        |
| **文档完整度**   | 完整 API 文档    | 基础 README   | 文档缺失    |
| **测试覆盖率**   | >80%            | >50%          | <50%        |
| **Clippy 警告**  | 0 警告          | <10 警告       | >10 警告     |

#### 3. 兼容性 (Compatibility) - 权重 20%

| 指标              | ✅ 优秀      | 🟡 良好    | ❌ 警告    |
| ----------------- | ------------ | ---------- | --------- |
| **Rust Edition**  | 2021         | 2018       | 更旧版本   |
| **MSRV**          | <1.70        | <1.60      | >1.60     |
| **依赖数量**      | <20 个       | <50 个      | >50 个     |
| ** Breaking Changes** | 0 次 (major) | 1 次       | >1 次      |

#### 4. 社区活跃度 (Community) - 权重 10%

| 指标            | ✅ 优秀      | 🟡 良好    | ❌ 警告    |
| --------------- | ------------ | ---------- | --------- |
| **贡献者数量**  | >10 人        | >5 人       | <5 人      |
| **月下载量**    | >10k         | >1k        | <1k       |
| **Discord/论坛** | 活跃社区     | 有限讨论   | 无社区    |

### 综合评分计算

```
总分 = (维护得分 × 0.40) + (质量得分 × 0.30) + (兼容性得分 × 0.20) + (社区得分 × 0.10)
```

### 推荐等级

- **🟢 Recommended (推荐)**: 总分 >80%
  - 可以安全使用
  - 文档完善
  - 社区活跃
  
- **🟡 Use with Caution (谨慎使用)**: 总分 50-80%
  - 功能可用但有局限
  - 需要自行评估风险
  
- **🔴 Avoid (避免使用)**: 总分 <50%
  - 可能已废弃
  - 存在严重问题
  - 寻找替代方案

### 评估模板

```markdown
## 库名：[crate-name]

**评分**: 🟢 85/100

### 维护状态 (40%)
- 最后更新：2 个月前 ✅
- 未解决问题：23 个 ✅
- 响应时间：3 天 ✅

### 代码质量 (30%)
- Stars: 1.2k ✅
- 文档：完整 API 文档 ✅
- 测试：87% 覆盖率 ✅

### 兼容性 (20%)
- Edition: Rust 2021 ✅
- MSRV: 1.65 ✅

### 社区 (10%)
- 贡献者：15 人 ✅
- 月下载：25k ✅

### 使用建议
适用于 [具体场景]，替代 [其他库] 以获得更好性能。
```

---

## 二、操作分类标准 (Operation Categories)

用于 SC-011 "代码片段速查" 的 10 个必备分类。

### 分类列表

每个分类必须包含 **5+ 个代码片段**。

#### 1. 文件操作 (File I/O)

```rust
// 1.1 读取整个文件
let content = std::fs::read_to_string("file.txt")?;

// 1.2 写入文件
std::fs::write("output.txt", "data")?;

// 1.3 追加内容
use std::fs::OpenOptions;
OpenOptions::new()
    .append(true)
    .open("log.txt")?
    .write_all(b"new line\n")?;

// 1.4 复制文件
std::fs::copy("src.txt", "dst.txt")?;

// 1.5 删除文件
std::fs::remove_file("file.txt")?;
```

#### 2. 集合操作 (Collections)

```rust
// 2.1 Vector 创建与添加
let mut nums = Vec::new();
nums.push(1);
nums.push(2);

// 2.2 迭代 Vector
for num in &nums {
    println!("{}", num);
}

// 2.3 Filter 和 Map
let doubled: Vec<i32> = nums.iter()
    .filter(|&&x| x > 1)
    .map(|&x| x * 2)
    .collect();

// 2.4 HashMap 插入
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");

// 2.5 HashMap 计数
for word in words {
    *map.entry(word).or_insert(0) += 1;
}
```

#### 3. 字符串操作 (String Operations)

```rust
// 3.1 创建字符串
let s = String::from("hello");

// 3.2 拼接字符串
let combined = format!("{} {}", s1, s2);

// 3.3 分割字符串
let parts: Vec<&str> = s.split(',').collect();

// 3.4 替换子串
let replaced = s.replace("old", "new");

// 3.5 解析数字
let num: i32 = "42".parse()?;
```

#### 4. 错误处理 (Error Handling)

```rust
// 4.1 Result 基本使用
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除数不能为 0"))
    } else {
        Ok(a / b)
    }
}

// 4.2 ? 操作符
fn read_file() -> std::io::Result<String> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}

// 4.3 unwrap 与 expect
let value = option.unwrap(); // panic if None
let value = option.expect("自定义错误信息");

// 4.4 match 处理错误
match result {
    Ok(v) => println!("成功：{}", v),
    Err(e) => println!("失败：{}", e),
}

// 4.5 map_err 转换错误
let num = "42".parse::<i32>()
    .map_err(|e| format!("解析失败：{}", e))?;
```

#### 5. 迭代器模式 (Iteration Patterns)

```rust
// 5.1 for 循环
for i in 0..10 {
    println!("{}", i);
}

// 5.2 迭代器
let sum: i32 = nums.iter().sum();

// 5.3 fold
let product = nums.iter().fold(1, |acc, &x| acc * x);

// 5.4 collect
let squared: Vec<i32> = nums.iter().map(|&x| x * x).collect();

// 5.5 find
let first_even = nums.iter().find(|&&x| x % 2 == 0);
```

#### 6. 特征实现 (Trait Implementations)

```rust
// 6.1 定义特征
trait Printable {
    fn print(&self);
}

// 6.2 实现特征
impl Printable for MyStruct {
    fn print(&self) {
        println!("{}", self.field);
    }
}

// 6.3 使用 derive
#[derive(Debug, Clone, PartialEq)]
struct MyStruct {
    field: i32,
}

// 6.4 特征作为参数
fn process(item: &impl Printable) {
    item.print();
}

// 6.5 特征 bound
fn process_all<T: Printable>(items: &[T]) {
    for item in items {
        item.print();
    }
}
```

#### 7. 并发原语 (Concurrency Primitives)

```rust
// 7.1 创建线程
use std::thread;
let handle = thread::spawn(|| {
    println!("子线程");
});
handle.join().unwrap();

// 7.2 通道消息传递
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send("hello").unwrap();
let msg = rx.recv().unwrap();

// 7.3 Mutex 保护共享数据
use std::sync::Mutex;
let data = Mutex::new(0);
*data.lock().unwrap() += 1;

// 7.4 Arc 多线程共享
use std::sync::Arc;
let shared = Arc::new(Mutex::new(0));
let clone1 = Arc::clone(&shared);

// 7.5 多线程示例
let mut handles = vec![];
for i in 0..5 {
    handles.push(thread::spawn(move || {
        println!("线程 {}", i);
    }));
}
```

#### 8. 测试模式 (Testing Patterns)

```rust
// 8.1 单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}

// 8.2 集成测试
// tests/integration_test.rs
#[test]
fn test_full_flow() {
    // 测试完整流程
}

// 8.3 测试错误情况
#[test]
fn test_divide_by_zero() {
    let result = divide(10, 0);
    assert!(result.is_err());
}

// 8.4 基准测试
#[bench]
fn bench_large_computation(b: &mut Bencher) {
    b.iter(|| {
        // 性能测试代码
    });
}

// 8.5 测试辅助函数
fn setup_test_data() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}
```

#### 9. 泛型与约束 (Generics & Bounds)

```rust
// 9.1 泛型函数
fn identity<T>(x: T) -> T {
    x
}

// 9.2 多类型参数
fn pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

// 9.3 Trait bound
fn print_all<T: std::fmt::Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

// 9.4 where 子句
fn complex_fn<T, U>(t: T, u: U) 
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
{
    // 实现
}

// 9.5 返回类型实现 trait
fn create() -> impl Iterator<Item = i32> {
    0..10
}
```

#### 10. 宏基础 (Macro Basics)

```rust
// 10.1 声明宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}
say_hello!();

// 10.2 带参数宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?}", stringify!($func_name));
        }
    };
}
create_function!(my_func);

// 10.3 可变参数
macro_rules! printlnall {
    ($($arg:expr),*) => {
        $(println!("{}", $arg);)*
    };
}
printlnall!(1, "hello", 3.14);

// 10.4 派生宏使用
#[derive(Debug, Clone)]
struct MyStruct {
    field: i32,
}

// 10.5 属性宏
#[tokio::main]
async fn main() {
    // 异步主函数
}
```

---

## 三、难度等级标准 (Difficulty Levels)

用于 SC-017 章节难度标记。

### 🟢 入门级 (Beginner)

**特征**:
- ✅ 无需前置章节或仅需前一章
- ✅ 单一概念焦点
- ✅ 示例直接明了
- ✅ 最小化错误处理
- ✅ 代码量 <100 行

**适用章节**:
- 变量与表达式
- 基础数据类型
- 结构体基础
- 枚举基础

**学习者准备**:
- 有基础编程概念
- 了解变量、循环、函数
- 无 Rust 经验要求

### 🟡 中级 (Intermediate)

**特征**:
- ✅ 需要 2+ 前置章节
- ✅ 组合多个概念
- ✅ 现实世界场景
- ✅ 适当的错误处理
- ✅ 代码量 100-300 行

**适用章节**:
- 所有权系统 (需要变量基础)
- 特征与泛型 (需要结构体基础)
- 闭包 (需要函数基础)
- 模块系统 (需要项目概念)

**学习者准备**:
- 完成入门章节
- 理解基础 Rust 语法
- 能编写简单程序

### 🔴 高级 (Advanced)

**特征**:
- ✅ 需要 5+ 前置章节
- ✅ 复杂交互与约束
- ✅ 边界情况与陷阱
- ✅ 性能考量
- ✅ 包含 unsafe 代码 (如适用)
- ✅ 代码量 >300 行

**适用章节**:
- 线程与并发
- 异步编程
- unsafe Rust
- 宏编程
- 生命周期高级

**学习者准备**:
- 完成中级章节
- 熟悉 Rust 核心概念
- 有实际项目经验
- 理解内存模型

### 难度评估流程

每章发布前评估：

1. **前置章节数量**: 
   - 0-1 章 → 🟢
   - 2-4 章 → 🟡
   - 5+ 章 → 🔴

2. **概念复杂度**:
   - 单一概念 → 🟢
   - 2-3 概念组合 → 🟡
   - 4+ 概念交互 → 🔴

3. **代码量**:
   - <100 行 → 🟢
   - 100-300 行 → 🟡
   - >300 行 → 🔴

4. **错误处理**:
   - 最小化 → 🟢
   - 适当处理 → 🟡
   - 全面处理 + 边界情况 → 🔴

**综合判定**: 满足 2 项及以上即升级。

---

## 四、章节内容审核清单

### 发布前检查 (每章必须)

- [ ] **代码质量**
  - [ ] 代码可编译 (`cargo build`)
  - [ ] 无 clippy 警告
  - [ ] 示例来自真实项目
  - [ ] GitHub 链接有效

- [ ] **教学结构** (12 节)
  - [ ] 开篇故事 (2-4 句类比)
  - [ ] 本章适合谁
  - [ ] 你会学到什么 (3-5 个目标)
  - [ ] 前置要求
  - [ ] 第一个例子 (<15 行)
  - [ ] 原理解析
  - [ ] 常见错误 (2-3 个)
  - [ ] 动手练习 (2-3 个)
  - [ ] 故障排查 (FAQ)
  - [ ] 知识扩展 (选学)
  - [ ] 小结 (3-5 要点)
  - [ ] 术语表

- [ ] **扩展功能** (SC-010 到 SC-018)
  - [ ] 延伸阅读 (3+ 库，带选择建议)
  - [ ] ASCII 图表 (关键概念)
  - [ ] 知识检查 (3+ 测验题)
  - [ ] 难度标记 (🟢🟡🔴)
  - [ ] 工业界应用案例

- [ ] **技术规范**
  - [ ] 术语中英文对照
  - [ ] 难度标记准确
  - [ ] 前置章节链接正确
  - [ ] unsafe 代码有明确警告

### 季度复审

每季度复审所有章节：

- [ ] 代码仍然可运行
- [ ] Crate 版本仍然兼容
- [ ] GitHub 链接仍然有效 (无 404)
- [ ] 无过时内容
- [ ] 库推荐仍然活跃 (重新评估)

---

## 五、质量指标

### 编译通过率
**目标**: 100%

所有代码示例必须通过：
```bash
cargo build --workspace
```

### 链接有效率
**目标**: >98%

随机抽查 20 个 GitHub 链接，计算有效率。

### 练习覆盖率
**目标**: 每章≥3 个

- 动手练习：2-3 个
- 知识检查：3 个
- 延伸阅读：提及 3+ 库

### 图表覆盖率
**目标**: 关键概念 100%

必须包含 ASCII 图表的概念：
- 所有权转移
- 借用规则
- 内存布局 (栈 vs 堆)
- 生命周期重叠
- 特征对象

### 用户满意度 (未来指标)
**目标**: >4.5/5.0

通过 GitHub issues、社区反馈收集。

---

**版本历史**:
- v1.0 (2026-04-04): 初始版本，包含库选择、操作分类、难度等级标准

**维护者**: Hello Rust 文档团队
**贡献**: 欢迎通过 PR 改进审核标准
