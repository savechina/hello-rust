# 阶段复习：基础部分

## 开篇故事

想象你刚学完驾驶理论——你知道交通规则、标志含义、操作步骤。但真正上路前，你需要一次综合练习：在模拟环境中把所有知识串联起来。阶段复习就是你的"驾驶模拟考"——把分散的概念整合成完整的能力。

---

## 本章适合谁

如果你已经完成了基础部分（第 1-10 章），现在想检验自己的学习成果，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 综合运用所有权、借用、生命周期知识
2. 识别和修复常见的 Rust 编译错误
3. 设计包含结构体、枚举、特征的系统
4. 理解模块可见性和代码组织

---

## 前置要求

完成以下章节：
- [变量与表达式](expression.md)
- [数据类型](datatype.md)
- [所有权](ownership.md)
- [结构体](struct.md)
- [枚举](enums.md)
- [特征](trait.md)
- [泛型](generic.md)
- [闭包](closure.md)
- [模块系统](module.md)
- [线程与并发](threads.md)

---

## 第一个例子

回顾所有权的核心模式：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 移动
    // println!("{}", s1);  // ❌ 错误
    
    let s3 = s2.clone();  // 克隆
    println!("{}, {}", s2, s3);  // ✅ 两者都可用
}
```

这个简单的例子涵盖了 Rust 最核心的概念：所有权转移和克隆。

---

## 原理解析

### 知识整合图

```
基础部分知识体系:

变量与表达式 ──→ 数据类型 ──→ 所有权 ──→ 结构体
                                         ↓
模块系统 ←── 线程与并发 ←── 闭包 ←── 泛型 ←── 特征 ←── 枚举
```

每个概念都建立在前一个概念之上，形成完整的知识链。

---

## 复习范围

第 1-10 章：变量与表达式、数据类型、所有权、结构体、枚举、特征、泛型、闭包、模块、线程

---

## 综合练习：设计一个简单的游戏角色系统

### 练习 1：角色定义

```rust,ignore
// TODO: 定义 Character 结构体
// 字段：name (String), health (u32), level (u32), class (职业枚举)

// TODO: 定义 Class 枚举
// 变体：Warrior, Mage, Ranger

// TODO: 实现 Character::new() 构造函数
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
#[derive(Debug)]
enum Class {
    Warrior,
    Mage,
    Ranger,
}

#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    level: u32,
    class: Class,
}

impl Character {
    fn new(name: &str, class: Class) -> Self {
        Character {
            name: name.to_string(),
            health: 100,
            level: 1,
            class,
        }
    }
}
```
</details>

### 练习 2：角色行为

```rust,ignore
// TODO: 实现 Character 的方法
// - level_up() - 等级 +1，生命值 +10
// - take_damage(amount: u32) - 减少生命值
// - is_alive() -> bool - 检查是否存活
// - display(&self) - 打印角色信息
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
impl Character {
    fn level_up(&mut self) {
        self.level += 1;
        self.health += 10;
        println!("{} 升级到等级 {}!", self.name, self.level);
    }

    fn take_damage(&mut self, amount: u32) {
        if amount >= self.health {
            self.health = 0;
            println!("{} 被击败了!", self.name);
        } else {
            self.health -= amount;
            println!("{} 受到 {} 点伤害，剩余 {} 点生命值", 
                     self.name, amount, self.health);
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn display(&self) {
        println!("{} (等级 {}, {:?}) - 生命值: {}", 
                 self.name, self.level, self.class, self.health);
    }
}
```
</details>

### 练习 3：战斗系统

```rust,ignore
// TODO: 实现 attack 函数
// 两个角色互相攻击
// Warrior: 造成 15-25 点伤害 (随机)
// Mage: 造成 20-30 点伤害，但自己受到 5 点反噬
// Ranger: 造成 10-20 点伤害，如果先手则 +10

// 提示：使用 rand crate 生成随机数
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use rand::Rng;

fn attack(attacker: &mut Character, defender: &mut Character) {
    if !attacker.is_alive() || !defender.is_alive() {
        return;
    }
    
    let mut rng = rand::thread_rng();
    let damage = match attacker.class {
        Class::Warrior => rng.gen_range(15..26),
        Class::Mage => {
            let dmg = rng.gen_range(20..31);
            attacker.take_damage(5);  // 反噬
            dmg
        }
        Class::Ranger => rng.gen_range(10..21),
    };
    
    println!("{} 攻击 {}，造成 {} 点伤害!", 
             attacker.name, defender.name, damage);
    defender.take_damage(damage);
}
```
</details>

---

## 知识检查

### 问题 1：所有权转移

这段代码会编译通过吗？为什么？

```rust,ignore
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
}
```

<details>
<summary>点击查看答案</summary>

❌ **不会通过**。`s1` 的所有权已移动给 `s2`，`s1` 不再有效。

**修复方法**：
```rust,ignore
let s2 = s1.clone();  // 克隆
// 或
println!("{}", &s1);  // 先借用再移动
```
</details>

### 问题 2：借用规则

以下代码有什么问题？

```rust,ignore
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, {}", r1, r2, r3);
}
```

<details>
<summary>点击查看答案</summary>

❌ **编译错误**。不能同时存在不可变引用 (`r1`, `r2`) 和可变引用 (`r3`)。

**修复方法**：
```rust,ignore
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);  // 不可变引用使用完毕
let r3 = &mut s;  // 现在可以创建可变引用
```
</details>

### 问题 3：特征实现

为以下类型实现 `Display` trait：

```rust,ignore
struct Point {
    x: f64,
    y: f64,
}
```

<details>
<summary>点击查看答案</summary>

```rust,ignore
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
</details>

### 问题 4：闭包捕获

```rust,ignore
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
```

闭包 `equal_to_x` 如何捕获 `x`？

<details>
<summary>点击查看答案</summary>

**不可变借用**。闭包通过 `&x` 捕获 `x`，因为只需要读取 `x` 的值。

如果想强制移动捕获：
```rust,ignore
let equal_to_x = move |z| z == x;
```
</details>

### 问题 5：模块可见性

```rust,ignore
mod outer {
    mod inner {
        pub fn hello() {
            println!("Hello!");
        }
    }
    
    pub fn call_hello() {
        inner::hello();
    }
}

fn main() {
    outer::call_hello();
    // outer::inner::hello();  // ❌ 为什么不行？
}
```

<details>
<summary>点击查看答案</summary>

`inner` 模块本身是私有的（没有 `pub`），所以外部无法访问 `inner::hello()`，即使 `hello` 是 `pub` 的。

**修复方法**：
```rust,ignore
pub mod inner {  // 模块也需要 pub
    pub fn hello() { ... }
}
```
</details>

---

## 常见错误回顾

| 错误 | 原因 | 修复 |
|------|------|------|
| `use after move` | 所有权已转移 | 使用 `.clone()` 或借用 `&` |
| `cannot borrow as mutable` | 违反借用规则 | 确保同一时间只有一个可变引用 |
| `lifetime may not live long enough` | 生命周期不匹配 | 添加生命周期标注 `'a` |
| `trait not implemented` | 缺少 trait 实现 | 使用 `impl Trait for Type` |
| `private module` | 模块可见性不足 | 添加 `pub` 到模块声明 |

---

## 小结

**核心要点**：

1. **所有权**是 Rust 内存安全的核心
2. **借用规则**防止数据竞争
3. **特征**实现多态
4. **模块系统**组织代码
5. **复习是巩固知识的关键**

**关键术语**：

- **所有权 (Ownership)**: 对值的独占访问
- **借用 (Borrowing)**: 临时访问，不转移所有权
- **特征 (Trait)**: 接口定义
- **模块 (Module)**: 代码组织单元

---

## 术语表

| English | 中文 |
|---------|------|
| Ownership | 所有权 |
| Borrowing | 借用 |
| Trait | 特征 |
| Module | 模块 |
| Visibility | 可见性 |
| Closure | 闭包 |
| Generic | 泛型 |

---

## 继续学习

- 下一步：[高级进阶](../advance/advance-overview.md)
- 挑战：[项目实战](../projects/)
- 回顾：[基础入门](basic-overview.md)

> 💡 **记住**：复习是学习的重要部分。不要急于前进，确保每个概念都理解了！
