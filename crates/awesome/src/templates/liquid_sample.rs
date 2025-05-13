use std::task::Context;

use liquid::{ParserBuilder, ValueView};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

fn liquid_sample() -> Result<(), liquid::Error> {
    let template = r#"
    <ul>
        {% for item in items %}
            <li>{{ item }}</li>
        {% endfor %}
    </ul>
    "#;

    let parser = ParserBuilder::with_stdlib().build()?;
    let template = parser.parse(template)?;

    let mut context = liquid::object!({
        "items": vec!["apple", "banana", "orange"],
    });

    let output = template.render(&mut context)?;
    println!("{}", output);
    Ok(())
}

fn liquid_file_sample() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file("templates/liquid/data.tpl")
        .unwrap();

    // User结构体
    let user = User {
        id: 123,
        name: "Alice".to_string(),
    };

    let mut context = liquid::object!({
        "items": vec!["apple", "banana", "orange"],
        "user": &user,
    });

    let output = template.render(&mut context).unwrap();

    println!("{}", output);

    // assert_eq!(output, "Liquid! 4\n".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquid_sample() {
        liquid_sample().unwrap();
    }

    #[test]
    fn test_liquid_file_sample() {
        liquid_file_sample();
    }
}
