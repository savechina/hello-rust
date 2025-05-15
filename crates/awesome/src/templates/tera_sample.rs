use chrono::prelude::*;
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

fn tera_sample() -> Result<(), tera::Error> {
    // 1. 创建 Tera 实例并加载模板
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // 2. 创建 Context 对象并插入数据
    let mut context = Context::new();
    context.insert("title", &"My Webpage");
    context.insert("greeting", &"Hello");
    context.insert("name", &"World");
    context.insert("show_message", &true);
    context.insert("message", &"This is a test message.");
    context.insert("items", &vec!["apple", "banana", "orange"]);
    let now = Utc::now().naive_utc();
    context.insert("now", &now);

    // User结构体
    let user = User {
        id: 123,
        name: "Alice".to_string(),
    };
    context.insert("user", &user);

    // 3. 渲染模板
    let rendered = tera.render("tera/index.html", &context)?;

    // 4. 输出渲染后的 HTML
    println!("{}", rendered);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tera_test() {
        tera_sample();
    }

    #[test]
    fn test_crates_path() {
        let path = std::env::current_dir().unwrap();
        println!("Current directory: {:?}", path);
        let crates_path = path.join("crates");
        println!("Crates path: {:?}", crates_path);
        let tera_path = crates_path.join("awesome").join("templates");
        println!("Tera path: {:?}", tera_path);
        let tera_path_str = tera_path.to_str().unwrap();
        println!("Tera path string: {:?}", tera_path_str);
        let tera = Tera::new(&format!("{}/**/*", tera_path_str));
        match tera {
            Ok(_) => println!("Tera instance created successfully."),
            Err(e) => println!("Error creating Tera instance: {}", e),
        }

        let tera = Tera::new("templates/**/*");
        match tera {
            Ok(_) => println!("Tera instance created successfully."),
            Err(e) => println!("Error creating Tera instance: {}", e),
        }

        // data path
        let data_path = path.join("data");
        println!("Data path: {:?}", data_path);
        let data_path_str = data_path.to_str().unwrap();
        println!("Data path string: {:?}", data_path_str);
    }
}
