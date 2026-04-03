# 模板引擎

## 开篇故事

想象你需要发送 1000 封个性化的欢迎邮件，每封邮件包含用户名、注册日期、专属优惠码。如果手动替换每个变量，不仅耗时而且容易出错。模板引擎就像一个智能的邮件合并工具——你定义模板，它自动填充变量，生成最终内容。

---

## 本章适合谁

如果你需要生成 HTML 页面、配置文件、邮件内容、报告文档等文本内容，本章适合你。模板引擎是 Web 开发和内容生成的必备工具。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 Tera 模板引擎渲染 HTML
2. 使用 Liquid 模板生成配置文件
3. 使用 Pest 解析器构建自定义语法
4. 选择合适的模板引擎

---

## 前置要求

- Rust 基础语法
- 字符串处理
- Serde 序列化（可选）

---

## 第一个例子

最简单的 Tera 模板渲染：

```rust
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tera = Tera::new("templates/**/*.html")?;
    let mut context = Context::new();
    context.insert("name", &"Alice");
    
    let rendered = tera.render("hello.html", &context)?;
    println!("{}", rendered);
    Ok(())
}
```

模板文件 `templates/hello.html`:
```html
<h1>Hello, {{ name }}!</h1>
```

完整示例：crates/awesome/src/templates/tera_sample.rs

---

## 原理解析

### 模板引擎对比

| 引擎 | 语法 | 适用场景 |
|------|------|---------|
| Tera | Jinja2-like | HTML 页面、Web 应用 |
| Liquid | Shopify Liquid | 静态网站、配置文件 |
| Pest | PEG 解析器 | 自定义 DSL、语法解析 |

### Tera 核心功能

- 变量替换：`{{ variable }}`
- 条件判断：`{% if condition %} ... {% endif %}`
- 循环迭代：`{% for item in list %} ... {% endfor %}`
- 过滤器：`{{ name | upper }}`

### Liquid 优势

- 安全性强（沙箱环境）
- Shopify 生态标准
- 适合用户自定义模板

---

## 常见错误

### 错误 1: 模板文件路径错误

确保模板文件路径正确，使用绝对路径或相对于项目根目录。

### 错误 2: 变量未定义

使用 `context.insert()` 添加所有模板需要的变量。

---

## 知识检查

**问题 1**: 哪个模板引擎适合 HTML 页面渲染？

答案：Tera（Jinja2 语法，Web 友好）

**问题 2**: Liquid 模板引擎的主要优势是什么？

答案：安全性强，沙箱环境，适合用户自定义模板

---

## 小结

核心要点：Tera 用于 HTML、Liquid 用于安全模板、Pest 用于自定义语法

完整示例：
- crates/awesome/src/templates/tera_sample.rs
- crates/awesome/src/templates/liquid_sample.rs
- crates/awesome/src/templates/pest_sample.rs
