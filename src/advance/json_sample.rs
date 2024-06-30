//!
//! JSON Sample
//!

use std::io;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

/**
 * parse untype JSON data
 * return Value HashMap
 */
pub(crate) fn untyped_sample() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

/**
 *  Struct serialize to Json
 */
pub(crate) fn typed_sample() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

pub(crate) fn json_transcode_sample() {
    // A JSON input with plenty of whitespace.
    let input = r#"
      {
        "a boolean": true,
        "an array": [3, 2, 1]
      }
    "#;

    // A JSON deserializer. You can use any Serde Deserializer here.
    let mut deserializer = serde_json::Deserializer::from_str(input);

    // A compacted JSON serializer. You can use any Serde Serializer here.
    let mut serializer = serde_json::Serializer::new(io::stdout());

    // Prints `{"a boolean":true,"an array":[3,2,1]}` to stdout.
    // This line works with any self-describing Deserializer and any Serializer.
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

/**
 *
 */
pub(crate) fn json_process_sample() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
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
    fn test_json() {
        untyped_sample().unwrap();

        typed_sample().unwrap();

        json_transcode_sample();

        json_process_sample();
    }
}
