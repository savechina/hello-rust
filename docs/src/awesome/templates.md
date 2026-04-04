# 模板引擎

## 开篇故事

想象你需要发送 1000 封个性化的欢迎邮件，每封邮件包含用户名、注册日期、专属优惠码。如果手动替换每个变量，不仅耗时而且容易出错。模板引擎就像一个智能的邮件合并工具——你定义模板，它自动填充变量，生成最终内容。

在 Rust 生态中，有三种主流的模板方案：**Tera**（Web 渲染）、**Liquid**（安全模板）、**Pest**（自定义语法解析）。每种方案解决不同的问题。

---

## 本章适合谁

如果你需要生成 HTML 页面、配置文件、邮件内容、报告文档、或自定义 DSL，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 使用 Tera 模板引擎渲染 HTML（循环、条件、过滤器）
2. 使用 Liquid 模板生成安全的内容（内联 + 文件）
3. 使用 Pest 解析器构建自定义语法（PEG 语法）
4. 根据场景选择合适的模板引擎

---

## 前置要求

- Rust 基础语法
- 字符串处理
- Serde 序列化（用于 Tera 结构体渲染）

---

### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tera
cargo add liquid
cargo add pest
cargo add pest_derive
cargo add serde --features derive
cargo add chrono
```

---

## 第一个例子

最简单的 Tera 模板渲染：

```rust,ignore
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

完整示例：[crates/awesome/src/templates/tera_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/tera_sample.rs)

---

## 原理解析

### 模板引擎对比

| 引擎 | 语法 | 适用场景 | 安全性 |
|------|------|---------|--------|
| **Tera** | Jinja2-like | HTML 页面、Web 应用 | 中等 |
| **Liquid** | Shopify Liquid | 静态网站、用户自定义模板 | 高（沙箱） |
| **Pest** | PEG 解析器 | 自定义 DSL、语法解析 | N/A（解析器） |

### Tera 完整示例

Tera 是最常用的 Rust 模板引擎，语法类似 Jinja2/Python。

**完整代码示例**（来自 `tera_sample.rs`）：

```rust,ignore
use tera::{Context, Tera};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

fn tera_sample() -> Result<(), tera::Error> {
    // 1. 创建 Tera 实例并加载模板
    let tera = Tera::new("templates/**/*")?;

    // 2. 创建 Context 对象
    let mut context = Context::new();
    context.insert("title", &"My Webpage");
    context.insert("greeting", &"Hello");
    context.insert("name", &"World");
    context.insert("show_message", &true);
    context.insert("message", &"This is a test message.");
    context.insert("items", &vec!["apple", "banana", "orange"]);
    context.insert("now", &chrono::Utc::now().naive_utc());

    // 插入结构体（自动序列化）
    let user = User { id: 123, name: "Alice".to_string() };
    context.insert("user", &user);

    // 3. 渲染模板
    let rendered = tera.render("tera/index.html", &context)?;
    println!("{}", rendered);

    Ok(())
}
```

**模板文件 `tera/index.html`**：

```html
<!DOCTYPE html>
<html>
<head><title>{{ title }}</title></head>
<body>
    <h1>{{ greeting }}, {{ name }}!</h1>
    
    {# 条件判断 #}
    {% if show_message %}
        <p>{{ message }}</p>
    {% endif %}
    
    {# 循环迭代 #}
    <ul>
    {% for item in items %}
        <li>{{ item | upper }}</li>
    {% endfor %}
    </ul>
    
    {# 结构体访问 #}
    <p>用户：{{ user.name }} (ID: {{ user.id }})</p>
    
    {# 日期格式化 #}
    <p>时间：{{ now | date(format="%Y-%m-%d %H:%M") }}</p>
</body>
</html>
```

**Tera 核心功能**：
- 变量替换：`{{ variable }}`
- 条件判断：`{% if condition %} ... {% endif %}`
- 循环迭代：`{% for item in list %} ... {% endfor %}`
- 过滤器：`{{ name | upper }}`、`{{ date | date(format="%Y") }}`
- 结构体访问：`{{ user.name }}`

### Liquid 完整示例

Liquid 是 Shopify 开发的安全模板引擎，适合用户自定义模板场景。

**内联模板示例**（来自 `liquid_sample.rs`）：

```rust,ignore
use liquid::{ParserBuilder, ValueView};

fn liquid_sample() -> Result<(), liquid::Error> {
    // 1. 定义模板（内联字符串）
    let template = r#"
    <ul>
        {% for item in items %}
            <li>{{ item }}</li>
        {% endfor %}
    </ul>
    "#;

    // 2. 创建解析器
    let parser = ParserBuilder::with_stdlib().build()?;
    let template = parser.parse(template)?;

    // 3. 创建上下文
    let mut context = liquid::object!({
        "items": vec!["apple", "banana", "orange"],
    });

    // 4. 渲染
    let output = template.render(&mut context)?;
    println!("{}", output);
    Ok(())
}
```

**文件模板示例**：

```rust,ignore
fn liquid_file_sample() {
    // 1. 从文件加载模板
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file("templates/liquid/data.tpl")
        .unwrap();

    // 2. 准备数据
    let user = User { id: 123, name: "Alice".to_string() };
    let mut context = liquid::object!({
        "items": vec!["apple", "banana", "orange"],
        "user": &user,
    });

    // 3. 渲染
    let output = template.render(&mut context).unwrap();
    println!("{}", output);
}
```

**模板文件 `templates/liquid/data.tpl`**：

```liquid
<h1>Liquid Template</h1>
<p>用户：{{ user.name }} (ID: {{ user.id }})</p>
<ul>
{% for item in items %}
    <li>{{ item }}</li>
{% endfor %}
</ul>
```

**Liquid 优势**：
- 安全性强（沙箱环境，无法执行任意代码）
- Shopify 生态标准
- 适合用户自定义模板（如 CMS）

### Pest 完整示例

Pest 不是传统模板引擎，而是 PEG（Parsing Expression Grammar）解析器生成器。它用于构建自定义语法解析器。

**完整代码示例**（来自 `pest_sample.rs`）：

```rust,ignore
use pest::Parser;
use pest_derive::Parser;

// 1. 定义解析器（使用 derive 宏加载语法文件）
#[derive(Parser)]
#[grammar = "templates/pest/grammar.pest"]
pub struct ExpressionParser;

fn pest_sample() {
    let input = "1 + 2 * 3";
    
    // 2. 解析输入
    let pairs = ExpressionParser::parse(Rule::expression, input).unwrap();

    // 3. 遍历解析结果
    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        println!("Span: {:?}", pair.as_span());
        println!("Text: {}", pair.as_str());
    }
}
```

**语法文件 `templates/pest/grammar.pest`**：

```pest
// 定义表达式语法规则
expression = _{ term ~ (("+" | "-") ~ term)* }
term = _{ factor ~ (("*" | "/") ~ factor)* }
factor = _{ number | "(" ~ expression ~ ")" }
number = @{ ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" }
```

**Pest 适用场景**：
- 自定义 DSL（领域特定语言）
- 配置文件解析
- 查询语言解析
- 数学表达式解析

---

## 常见错误

### 错误 1: 模板文件路径错误

确保模板文件路径正确，使用相对于 Cargo.toml 的路径：

```rust,ignore
// ❌ 错误：绝对路径
let tera = Tera::new("/absolute/path/templates/**/*.html")?;

// ✅ 正确：相对路径
let tera = Tera::new("templates/**/*.html")?;
```

### 错误 2: 变量未定义

使用 `context.insert()` 添加所有模板需要的变量：

```rust,ignore
// ❌ 错误：模板使用 {{ name }} 但未插入
let rendered = tera.render("hello.html", &context)?;

// ✅ 正确：先插入变量
context.insert("name", &"Alice");
let rendered = tera.render("hello.html", &context)?;
```

### 错误 3: Pest 语法文件路径

Pest 的 `#[grammar]` 路径相对于 `src/` 目录：

```rust,ignore
// ❌ 错误：路径不正确
#[grammar = "grammar.pest"]

// ✅ 正确：相对于 src/
#[grammar = "templates/pest/grammar.pest"]
```

---

## 知识检查

**问题 1**: 哪个模板引擎适合 HTML 页面渲染？

**问题 2**: Liquid 模板引擎的主要优势是什么？

**问题 3**: Pest 适合什么场景？

<details>
<summary>点击查看答案与解析</summary>

1. Tera（Jinja2 语法，Web 友好，支持结构体序列化）
2. 安全性强，沙箱环境，适合用户自定义模板（如 CMS）
3. 自定义 DSL、语法解析、配置文件解析（PEG 解析器）

**关键理解**: 选择模板引擎取决于你的需求：Web 用 Tera，安全用 Liquid，解析用 Pest。
</details>

---

## 延伸阅读

学习完模板引擎后，你可能还想了解：

- [Askama](https://docs.rs/askama) - 编译时模板（零运行时开销）
- [Handlebars](https://docs.rs/handlebars) - JavaScript Handlebars 的 Rust 实现
- [Minijinja](https://docs.rs/minijinja) - 轻量级 Jinja2 实现

**选择建议**:
- Web 应用 → Tera
- 用户自定义模板 → Liquid
- 自定义语法 → Pest
- 极致性能 → Askama（编译时渲染）

---

## 小结

核心要点：Tera 用于 HTML、Liquid 用于安全模板、Pest 用于自定义语法

完整示例：
- [tera_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/tera_sample.rs)
- [liquid_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/liquid_sample.rs)
- [pest_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/templates/pest_sample.rs)
