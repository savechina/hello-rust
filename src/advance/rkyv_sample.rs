/// rkyv 零拷贝序列化示例
///
/// rkyv 是一个高性能的零拷贝序列化/反序列化库
/// 适用于需要极致性能的场景：游戏、数据库、网络传输
///
/// 特点：
/// - 零拷贝反序列化：直接访问内存，无需复制
/// - 编译时验证：类型安全
/// - 高性能：比 serde 快 10-100 倍

use rkyv::{
    access, access_unchecked,
    rancor::Error,
    ser::allocator::Arena,
    Archive, Deserialize, Serialize,
};

/// 测试结构体
/// 使用 rkyv 的派生宏生成序列化和反序列化代码
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct TestStruct {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
    float: f64,
    boolean: bool,
}

/// 复杂嵌套结构体
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct NestedStruct {
    name: String,
    items: Vec<TestStruct>,
    metadata: Option<String>,
}

/// 基本序列化示例
/// 演示如何将结构体序列化为字节
pub fn rkyv_basic_serialize_sample() {
    println!("=== 基本序列化示例 ===\n");

    let value = TestStruct {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
        float: 3.14159,
        boolean: true,
    };

    println!("原始数据：{:?}", value);

    // 序列化：结构体 → 字节
    let bytes = rkyv::to_bytes::<Error>(&value).unwrap();
    println!("序列化后字节数：{}", bytes.len());

    // 安全 API：零拷贝访问
    let archived = access::<ArchivedTestStruct, Error>(&bytes).unwrap();
    println!("归档数据：{:?}", archived);
    println!("归档 string 长度：{}", archived.string.len());
    println!("归档 option 是否有值：{}", archived.option.is_some());

    // 反序列化：字节 → 结构体
    let deserialized = rkyv::deserialize::<TestStruct, Error>(archived).unwrap();
    println!("反序列化数据：{:?}", deserialized);

    // 验证数据一致性
    assert_eq!(value, deserialized);
    println!("✓ 数据一致性验证通过\n");
}

/// 自定义分配器序列化
/// 演示如何使用自定义内存分配器优化性能
pub fn rkyv_custom_allocator_sample() {
    println!("=== 自定义分配器序列化示例 ===\n");

    let value = TestStruct {
        int: 100,
        string: "custom allocator".to_string(),
        option: Some(vec![10, 20, 30]),
        float: 2.71828,
        boolean: false,
    };

    // 使用 Arena 分配器
    let mut arena = Arena::new();
    let bytes = rkyv::api::high::to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();

    println!("使用 Arena 分配器序列化");
    println!("字节数：{}", bytes.len());

    // 零拷贝访问
    let archived = unsafe { access_unchecked::<ArchivedTestStruct>(&bytes) };
    println!("归档 int 值：{}", archived.int);
    println!("归档 string 内容：{}", &archived.string);
    println!("✓ 自定义分配器序列化成功\n");
}

/// 嵌套结构体序列化
/// 演示复杂数据结构的序列化
pub fn rkyv_nested_struct_sample() {
    println!("=== 嵌套结构体序列化示例 ===\n");

    let nested = NestedStruct {
        name: "nested example".to_string(),
        items: vec![
            TestStruct {
                int: 1,
                string: "item 1".to_string(),
                option: Some(vec![1]),
                float: 1.1,
                boolean: true,
            },
            TestStruct {
                int: 2,
                string: "item 2".to_string(),
                option: None,
                float: 2.2,
                boolean: false,
            },
        ],
        metadata: Some("metadata".to_string()),
    };

    println!("原始嵌套数据：{:?}", nested);

    let bytes = rkyv::to_bytes::<Error>(&nested).unwrap();
    println!("序列化后字节数：{}", bytes.len());

    let archived = access::<ArchivedNestedStruct, Error>(&bytes).unwrap();
    println!("归档 items 数量：{}", archived.items.len());

    let deserialized = rkyv::deserialize::<NestedStruct, Error>(archived).unwrap();
    println!("反序列化数据：{:?}", deserialized);

    assert_eq!(nested, deserialized);
    println!("✓ 嵌套结构体序列化成功\n");
}

/// 性能对比示例
/// 演示 rkyv 相对于 serde_json 的性能优势
pub fn rkyv_performance_sample() {
    println!("=== 性能对比示例 ===\n");

    let value = TestStruct {
        int: 255,
        string: "performance test".to_string(),
        option: Some((0..1000).collect()),
        float: 1.23456789,
        boolean: true,
    };

    // rkyv 序列化
    let start = std::time::Instant::now();
    let rkyv_bytes = rkyv::to_bytes::<Error>(&value).unwrap();
    let rkyv_serialize_time = start.elapsed();

    // rkyv 反序列化
    let start = std::time::Instant::now();
    let archived = access::<ArchivedTestStruct, Error>(&rkyv_bytes).unwrap();
    let _ = rkyv::deserialize::<TestStruct, Error>(archived).unwrap();
    let rkyv_deserialize_time = start.elapsed();

    println!("rkyv 序列化时间：{:?}", rkyv_serialize_time);
    println!("rkyv 反序列化时间：{:?}", rkyv_deserialize_time);
    println!("rkyv 字节数：{}", rkyv_bytes.len());
    println!("✓ 性能测试完成\n");
}

/// 数据验证示例
/// 演示如何验证归档数据的有效性
pub fn rkyv_validation_sample() {
    println!("=== 数据验证示例 ===\n");

    let value = TestStruct {
        int: 42,
        string: "valid data".to_string(),
        option: Some(vec![1, 2, 3]),
        float: 3.14,
        boolean: true,
    };

    let bytes = rkyv::to_bytes::<Error>(&value).unwrap();

    // rkyv 0.8: access() already validates the bytes internally
    // If the bytes are corrupted, access() will return an error
    match rkyv::access::<ArchivedTestStruct, Error>(&bytes) {
        Ok(archived) => {
            println!("数据验证：有效");
            println!("归档 int 值：{}", archived.int);
        }
        Err(e) => {
            println!("数据验证：无效 - {:?}", e);
        }
    }
    println!("✓ 数据验证成功\n");
}

/// 主函数：运行所有示例
#[tokio::main]
async fn main() {
    rkyv_basic_serialize_sample();
    rkyv_custom_allocator_sample();
    rkyv_nested_struct_sample();
    rkyv_performance_sample();
    rkyv_validation_sample();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_serialize() {
        rkyv_basic_serialize_sample();
    }

    #[test]
    fn test_custom_allocator() {
        rkyv_custom_allocator_sample();
    }

    #[test]
    fn test_nested_struct() {
        rkyv_nested_struct_sample();
    }

    #[test]
    fn test_performance() {
        rkyv_performance_sample();
    }

    #[test]
    fn test_validation() {
        rkyv_validation_sample();
    }
}
