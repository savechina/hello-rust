# 阶段复习：基础部分

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

## 下一步

完成基础部分复习后，你可以：

1. 继续学习 **[高级进阶](../advance/advance-overview.md)**
2. 挑战 **[项目实战](../projects/README.md)**
3. 回顾不确定的概念，重新阅读相关章节

> 💡 **记住**：复习是学习的重要部分。不要急于前进，确保每个概念都理解了！
