# Ollama AI 集成

## 开篇故事

想象你要和一个 AI 聊天。传统方式是：调用复杂的 API → 处理响应 → 显示结果。Ollama 就像是：本地 AI 助手——在本地运行大语言模型，简单快速地集成 AI 功能。

---

## 本章适合谁

如果你想在 Rust 程序中集成 AI 功能（聊天机器人、文本生成），本章适合你。Ollama 是本地运行大语言模型的简单方式。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Ollama 概念
2. 连接 Ollama 服务
3. 发送生成请求
4. 处理 AI 响应
5. 创建聊天机器人

---

## 前置要求

- [异步编程](async.md) - async/await 基础
- [Tokio 运行时](async.md) - Tokio 基础
- [错误处理](error-handling.md) - 错误处理基础

---

## 第一个例子

最简单的 Ollama 使用：

```rust
use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接本地 Ollama 服务（默认 localhost:11434）
    let ollama = Ollama::default();
    
    // 指定模型和提示
    let model = "llama3.2:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();
    
    // 发送生成请求
    let res = ollama.generate(
        GenerationRequest::new(model, prompt)
    ).await?;
    
    // 打印响应
    if let Ok(res) = res {
        println!("{}", res.response);
    }
    
    Ok(())
}
```

**完整示例**: [ollama_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/ollama_sample.rs)

---

## 原理解析

### Ollama 特性

**Ollama 是本地 AI 运行库**：

- ✅ 本地运行大语言模型
- ✅ 简单易用的 API
- ✅ 支持多种模型
- ✅ 异步支持

### 连接 Ollama

**使用默认配置**：

```rust
use ollama_rs::Ollama;

// 默认连接到 localhost:11434
let ollama = Ollama::default();
```

**使用自定义配置**：

```rust
// 自定义主机和端口
let ollama = Ollama::new(
    "http://localhost".to_string(),
    11434
);
```

### 生成请求

**使用 GenerationRequest**：

```rust
use ollama_rs::generation::completion::request::GenerationRequest;

let model = "llama3.2:latest".to_string();
let prompt = "解释量子力学".to_string();

let request = GenerationRequest::new(model, prompt);
let response = ollama.generate(request).await?;
```

### 处理响应

**解析生成结果**：

```rust
let res = ollama.generate(request).await?;

if let Ok(res) = res {
    println!("AI 响应：{}", res.response);
} else {
    eprintln!("生成失败");
}
```

### 聊天机器人

**实现简单聊天**：

```rust
async fn chatbot() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "llama3.2:latest".to_string();
    
    loop {
        print!("你：");
        std::io::stdout().flush()?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if input.trim() == "exit" {
            break;
        }
        
        let response = ollama.generate(
            GenerationRequest::new(model.clone(), input.trim().to_string())
        ).await?;
        
        println!("AI: {}", response.response);
    }
    
    Ok(())
}
```

---

## 常见错误

### 错误 1: Ollama 服务未运行

```rust
let ollama = Ollama::default();
let res = ollama.generate(request).await?;
// ❌ 如果 Ollama 服务未运行会失败
```

**错误信息**:
```
error sending request for url
```

**修复方法**:
```bash
# 启动 Ollama 服务
ollama serve
```

### 错误 2: 模型未下载

```rust
let model = "llama3.2:latest".to_string();
// ❌ 如果模型未下载会失败
```

**错误信息**:
```
model 'llama3.2:latest' not found
```

**修复方法**:
```bash
# 下载模型
ollama pull llama3.2:latest
```

### 错误 3: 忘记 await

```rust
let res = ollama.generate(request);  // ❌ 忘记 .await
println!("{}", res.response);
```

**错误信息**:
```
no field `response` on type `impl Future`
```

**修复方法**:
```rust
let res = ollama.generate(request).await?;  // ✅ 添加 .await
```

---

## 动手练习

### 练习 1: 连接 Ollama

```rust
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    // TODO: 创建 Ollama 实例
    // TODO: 打印连接信息
}
```

<details>
<summary>点击查看答案</summary>

```rust
let ollama = Ollama::default();
println!("已连接到 Ollama 服务");
```
</details>

### 练习 2: 发送问题

```rust
use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    
    // TODO: 创建请求
    // TODO: 发送问题 "什么是 Rust？"
    // TODO: 打印回答
}
```

<details>
<summary>点击查看答案</summary>

```rust
let model = "llama3.2:latest".to_string();
let prompt = "什么是 Rust？".to_string();

let res = ollama.generate(
    GenerationRequest::new(model, prompt)
).await.unwrap();

println!("{}", res.response);
```
</details>

### 练习 3: 创建聊天机器人

```rust
async fn chatbot() {
    let ollama = Ollama::default();
    let model = "llama3.2:latest".to_string();
    
    // TODO: 循环读取用户输入
    // TODO: 发送到 Ollama
    // TODO: 打印响应
    // TODO: 输入"exit"退出
}
```

<details>
<summary>点击查看答案</summary>

```rust
loop {
    print!("你：");
    std::io::stdout().flush().unwrap();
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if input.trim() == "exit" {
        break;
    }
    
    let response = ollama.generate(
        GenerationRequest::new(model.clone(), input.trim().to_string())
    ).await.unwrap();
    
    println!("AI: {}", response.response);
}
```
</details>

---

## 故障排查 (FAQ)

### Q: Ollama 和 OpenAI API 有什么区别？

**A**: 
- **Ollama**: 本地运行，免费，隐私好
- **OpenAI**: 云端 API，付费，功能强
- **选择**: 根据需求选择

### Q: 支持哪些模型？

**A**: 
- Llama 3.2
- Llama 3
- Mistral
- Gemma
- 等等（查看 ollama.ai 获取完整列表）

### Q: 如何提高响应速度？

**A**: 
- 使用较小的模型
- 减少提示长度
- 使用 GPU 加速

---

## 知识扩展

### 流式响应

```rust
use ollama_rs::generation::completion::request::GenerationRequest;

let mut stream = ollama.generate_stream(
    GenerationRequest::new(model, prompt)
).await?;

while let Some(chunk) = stream.next().await {
    print!("{}", chunk.response);
    std::io::stdout().flush()?;
}
```

### 多轮对话

```rust
let mut context = Vec::new();

// 第一轮
let response = ollama.generate(
    GenerationRequest::new(model.clone(), "你好")
        .with_context(context.clone())
).await?;
context.push(response.context);

// 第二轮（带上下文）
let response = ollama.generate(
    GenerationRequest::new(model, "继续")
        .with_context(context)
).await?;
```

### 自定义参数

```rust
let request = GenerationRequest::new(model, prompt)
    .temperature(0.8)  // 创造性
    .top_p(0.9)        // 采样
    .num_predict(100); // 最大 token 数
```

---

## 小结

**核心要点**：

1. **Ollama**: 本地 AI 运行库
2. **默认连接**: localhost:11434
3. **GenerationRequest**: 生成请求
4. **异步支持**: 使用 async/await
5. **聊天机器人**: 简单实现

**关键术语**：

- **Ollama**: AI 运行平台
- **Generation**: 生成
- **Model**: 模型
- **Prompt**: 提示

---

## 术语表

| English | 中文 |
| ------- | ---- |
| Ollama | Ollama |
| Generation | 生成 |
| Model | 模型 |
| Prompt | 提示 |
| Chatbot | 聊天机器人 |

---

## 知识检查

**快速测验**（答案在下方）：

1. Ollama 是什么？

2. 如何在 Rust 中调用 Ollama API？

3. 本地 AI 集成的应用场景有哪些？

<details>
<summary>点击查看答案与解析</summary>

1. Ollama 是本地运行大语言模型的工具
2. 使用 HTTP 客户端（如 reqwest）调用 Ollama 的 REST API
3. 智能助手、代码生成、文本分析、摘要

**关键理解**: 本地 AI 集成保护隐私，无需云端依赖。
</details>

## 继续学习

**前一章**: [对象存储](objectstore.md)  
**下一章**: [进程管理](process.md)

**相关章节**:
- [对象存储](objectstore.md)
- [异步编程](async.md)
- [Tokio 运行时](async.md)

**返回**: [高级进阶](advance-overview.md)

---

**完整示例**: [ollama_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/advance/ollama_sample.rs)
