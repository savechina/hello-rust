# RSpec 测试框架

## 开篇故事

想象你要写测试报告。传统方式是：写测试函数 → 断言 → 打印结果。RSpec 就像是：用自然语言描述测试——"describe 用户服务，it 应该创建用户，it 应该删除用户"。rspec crate 帮你用 BDD 风格写测试。

---

## 本章适合谁

如果你想用 BDD 风格编写测试（行为驱动开发），本章适合你。RSpec 让测试更像文档。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 BDD 测试概念
2. 使用 rspec crate
3. 使用 speculate 宏
4. 编写描述性测试
5. 组织测试套件

---

## 前置要求

- [测试基础](test.md) - 测试基础
- [Mock 模拟](mock.md) - Mock 基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add rspec --dev
cargo add speculate --dev
cargo add mockall --dev
```

## 第一个例子

最简单的 RSpec 使用：

```rust
use rspec::describe;
use rspec::suite::Suite;

#[test]
fn test_rspec_suite() {
    rspec::run(&describe(
        "monitor_listener",
        (),  // 测试上下文
        |ctx| {
            ctx.it("should call monitor service", |_| {
                // 测试代码
                assert!(true);
            });
            
            ctx.it("should verify monitor call", |_| {
                assert_eq!(1 + 1, 2);
            });
        },
    ));
}
```

**完整示例**: [rspec_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/rspec_sample.rs)

---

## 原理解析

### rspec 特性

**rspec 是 BDD 测试框架**：

- ✅ 描述性测试
- ✅ 嵌套结构
- ✅ 共享上下文
- ✅ 行为驱动

### 使用 describe

**使用 describe 组织测试**：

```rust
use rspec::describe;

describe("UserService", |ctx| {
    ctx.it("should create user", |_| {
        // 测试创建用户
    });
    
    ctx.it("should delete user", |_| {
        // 测试删除用户
    });
});
```

### 使用 speculate 宏

**使用 speculate! 宏**：

```rust
use speculate::speculate;

speculate! {
    describe "UserService" {
        before {
            // 每个测试前执行
            let service = UserService::new();
        }
        
        it "should create user" {
            assert!(service.create("Alice"));
        }
        
        it "should delete user" {
            assert!(service.delete("Alice"));
        }
    }
}
```

### 共享上下文

**使用 before 块**：

```rust
speculate! {
    describe "Database" {
        before {
            let db = Database::connect("test.db");
        }
        
        it "should connect" {
            assert!(db.is_connected());
        }
        
        it "should query" {
            let results = db.query("SELECT *");
            assert!(!results.is_empty());
        }
    }
}
```

### 嵌套描述

**嵌套 describe**：

```rust
speculate! {
    describe "UserService" {
        describe "create" {
            it "should create valid user" {
                // ...
            }
            
            it "should reject invalid email" {
                // ...
            }
        }
        
        describe "delete" {
            it "should delete existing user" {
                // ...
            }
        }
    }
}
```

---

## 常见错误

### 错误 1: 忘记导入

```rust
speculate! {
    describe "Test" {
        it "should work" {
            assert!(true);
        }
    }
}
// ❌ 忘记 use speculate::speculate;
```

**错误信息**:
```
cannot find macro `speculate`
```

**修复方法**:
```rust
use speculate::speculate;  // ✅ 导入宏
```

### 错误 2: 语法错误

```rust
speculate! {
    describe "Test" {
        it "should work" {
            assert!(true);
        // ❌ 忘记闭合花括号
```

**错误信息**:
```
unexpected end of macro invocation
```

**修复方法**:
```rust
speculate! {
    describe "Test" {
        it "should work" {
            assert!(true);
        }
    }
}  // ✅ 闭合所有括号
```

### 错误 3: 共享变量作用域

```rust
speculate! {
    describe "Test" {
        let shared = 42;  // ❌ 变量作用域错误
        
        it "test1" {
            println!("{}", shared);
        }
    }
}
```

**修复方法**:
```rust
speculate! {
    describe "Test" {
        before {
            let shared = 42;  // ✅ 在 before 块中定义
        }
        
        it "test1" {
            println!("{}", shared);
        }
    }
}
```

---

## 动手练习

### 练习 1: 简单测试

```rust
use speculate::speculate;

speculate! {
    describe "Calculator" {
        // TODO: 测试加法
        // TODO: 测试减法
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
speculate! {
    describe "Calculator" {
        it "should add" {
            assert_eq!(2 + 2, 4);
        }
        
        it "should subtract" {
            assert_eq!(5 - 3, 2);
        }
    }
}
```
</details>

### 练习 2: 使用 before 块

```rust
speculate! {
    describe "UserService" {
        // TODO: 在 before 中创建服务
        // TODO: 测试创建用户
        // TODO: 测试查询用户
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
speculate! {
    describe "UserService" {
        before {
            let service = UserService::new();
        }
        
        it "should create user" {
            assert!(service.create("Alice"));
        }
        
        it "should find user" {
            let user = service.find("Alice");
            assert!(user.is_some());
        }
    }
}
```
</details>

### 练习 3: 嵌套描述

```rust
speculate! {
    // TODO: 描述 "Database"
    //   TODO: 描述 "connect"
    //     TODO: 测试成功连接
    //   TODO: 描述 "query"
    //     TODO: 测试查询结果
}
```

<details>
<summary>点击查看答案</summary>

```rust
speculate! {
    describe "Database" {
        describe "connect" {
            it "should connect successfully" {
                assert!(db.connect().is_ok());
            }
        }
        
        describe "query" {
            it "should return results" {
                let results = db.query("SELECT *");
                assert!(!results.is_empty());
            }
        }
    }
}
```
</details>

---

## 故障排查 (FAQ)

### Q: RSpec 和标准测试有什么区别？

**A**: 
- **标准测试**: `#[test] fn test_xxx()`
- **RSpec**: 描述性，BDD 风格
- **优势**: RSpec 更像文档，易读

### Q: 什么时候使用 RSpec？

**A**: 
- 需要行为驱动测试
- 测试需要清晰描述
- 团队熟悉 BDD

### Q: RSpec 会影响性能吗？

**A**: 
- 运行时性能无影响（仅测试代码）
- 编译时间略长（宏展开）
- 测试执行速度相同

---

## 知识扩展

### 自定义匹配器

```rust
speculate! {
    describe "Custom Matcher" {
        it "should be even" {
            let num = 4;
            assert!(num % 2 == 0, "expected even number");
        }
    }
}
```

### 跳过测试

```rust
speculate! {
    describe "Skipped" {
        xit "should skip this test" {
            // 这个测试会被跳过
        }
    }
}
```

### 条件测试

```rust
#[cfg(feature = "advanced")]
speculate! {
    describe "Advanced" {
        it "should work with feature" {
            // 只在启用 advanced 特性时运行
        }
    }
}
```

---

## 小结

**核心要点**：

1. **rspec**: BDD 测试框架
2. **describe**: 组织测试
3. **it**: 单个测试
4. **before**: 共享上下文
5. **speculate!**: 宏语法

**关键术语**：

- **BDD**: 行为驱动开发
- **Describe**: 描述块
- **It**: 测试用例
- **Before**: 前置块

---

## 术语表

| English | 中文 |
| ------- | ---- |
| BDD | 行为驱动开发 |
| Describe | 描述块 |
| It | 测试用例 |
| Before | 前置块 |
| Speculate | 推测宏 |

---

## 知识检查

**快速测验**（答案在下方）：

1. RSpec 风格测试和普通 Rust 测试有什么区别？

2. `describe` 和 `it` 的作用是什么？

3. 什么时候应该使用 BDD 风格测试？

<details>
<summary>点击查看答案与解析</summary>

1. RSpec 使用 `describe/it` 语法，更接近自然语言
2. `describe` = 测试组，`it` = 单个测试用例
3. 复杂业务逻辑、行为驱动开发、团队协作

**关键理解**: BDD 测试更易读，但需要额外依赖。
</details>

## 继续学习

**前一章**: [Mock 模拟](mock.md)  
**下一章**: [派生宏](getset.md)

**相关章节**:
- [Mock 模拟](mock.md)
- [测试基础](test.md)
- [派生宏](getset.md)

**返回**: 高级进阶

---

**完整示例**: [rspec_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/rspec_sample.rs)
