use futures::executor::block_on;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio::io;
#[tokio::main]
async fn chatbot() -> io::Result<()> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();

    // For custom values:
    // let ollama = Ollama::new("http://localhost".to_string(), 11434);

    let model = "llama3.2:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }

    Ok(())
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_fetures_hello() {
        chatbot();
    }
}
