use ollama_rs::{
    generation::{
        completion::{request::GenerationRequest, GenerationResponse},
        images::Image,
    },
    Ollama,
};
use tokio::io;

/// 基础聊天机器人示例
/// 使用默认 Ollama 服务 (localhost:11434)
#[tokio::main]
pub async fn ollama_chat_sample() -> io::Result<()> {
    let ollama = Ollama::default();
    let model = "llama3.2:latest".to_string();
    let prompt = "Why is the sky blue? Explain in simple terms.".to_string();

    println!("提问：{}", prompt);
    println!("思考中...\n");

    match ollama.generate(GenerationRequest::new(model, prompt)).await {
        Ok(res) => {
            println!("回答：\n{}", res.response);
            println!("\n--- 元数据 ---");
            println!("模型：{}", res.model);
            println!("总耗时：{:?}ms", res.total_duration);
            println!("总耗时：{:?}ns", res.total_duration);
            println!("评估计数：{}", res.eval_count.unwrap_or(0));
        }
        Err(e) => {
            eprintln!("生成失败：{}", e);
        }
    }

    Ok(())
}

/// 多轮对话示例
/// 展示如何在对话中保持上下文
#[tokio::main]
pub async fn ollama_conversation_sample() -> io::Result<()> {
    let ollama = Ollama::default();
    let model = "llama3.2:latest".to_string();

    // 第一轮对话
    let prompt1 = "What is Rust? Give a brief introduction.".to_string();
    println!("问题 1：{}", prompt1);

    if let Ok(res) = ollama
        .generate(GenerationRequest::new(model.clone(), prompt1))
        .await
    {
        println!("回答 1：\n{}\n", res.response);
    }

    // 第二轮对话（新的独立请求）
    let prompt2 = "What are the main advantages of Rust over C++?".to_string();
    println!("问题 2：{}", prompt2);

    if let Ok(res) = ollama
        .generate(GenerationRequest::new(model, prompt2))
        .await
    {
        println!("回答 2：\n{}", res.response);
    }

    Ok(())
}

/// 带图片的多模态示例
/// 使用支持视觉的模型分析图片
#[tokio::main]
pub async fn ollama_vision_sample() -> io::Result<()> {
    let ollama = Ollama::default();
    let model = "llava:latest".to_string();

    // 注意：需要实际图片文件路径
    // let image = Image::from_path("path/to/image.jpg")?;
    // let prompt = "What's in this image?".to_string();
    // let res = ollama.generate(GenerationRequest::new(model, prompt).add_image(image)).await;

    println!("多模态示例需要 llava 模型和实际图片");
    println!("安装模型：ollama pull llava");
    println!("然后提供图片路径运行");

    Ok(())
}

/// 流式响应示例
/// 逐 token 输出，适合长文本生成
#[tokio::main]
pub async fn ollama_stream_sample() -> io::Result<()> {
    use tokio_stream::StreamExt;

    let ollama = Ollama::default();
    let model = "llama3.2:latest".to_string();
    let prompt = "Write a short poem about programming in Rust.".to_string();

    println!("提问：{}\n", prompt);
    println!("流式响应：");

    let mut stream = ollama
        .generate_stream(GenerationRequest::new(model, prompt))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut full_response = String::new();

    while let Some(Ok(chunk)) = stream.next().await {
        // Each chunk is a Vec<GenerationResponse>
        for res in chunk {
            if !res.response.is_empty() {
                print!("{}", res.response);
                full_response.push_str(&res.response);
            }
        }
    }

    println!("\n\n--- 完整响应 ---");
    println!("{}", full_response);

    Ok(())
}

/// 错误处理示例
/// 展示如何处理各种 Ollama 错误
#[tokio::main]
pub async fn ollama_error_handling_sample() -> io::Result<()> {
    let ollama = Ollama::default();

    // 测试不存在的模型
    let model = "nonexistent_model".to_string();
    let prompt = "Hello".to_string();

    match ollama
        .generate(GenerationRequest::new(model, prompt))
        .await
    {
        Ok(res) => println!("成功：{}", res.response),
        Err(e) => {
            eprintln!("错误类型：{:?}", std::error::Error::source(&e));
            eprintln!("错误信息：{}", e);

            // 检查是否是模型未找到错误
            if e.to_string().contains("model") {
                eprintln!("提示：请运行 'ollama pull llama3.2' 安装模型");
            }
        }
    }

    Ok(())
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_ollama_chat() {
        // 需要本地运行 Ollama 服务
        let result = ollama_chat_sample().await;
        assert!(result.is_ok());
    }

    #[ignore]
    #[tokio::test]
    async fn test_ollama_error_handling() {
        // 测试错误处理
        let result = ollama_error_handling_sample().await;
        assert!(result.is_ok());
    }
}
