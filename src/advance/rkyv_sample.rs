/// This module demonstrates how to serialize a `TestStruct` using the `rkyv` crate.
/// The `rkyv` crate provides a zero-copy serialization of Rust types, which can be
/// used in any context that requires a `TestStruct`.
/// Allowing for zero-copy deserialization can be very useful in scenarios where memory efficiency is critical. For example, when working with large datasets or when transferring data over the network.
/// Archived types is `ArchiveTestStruct` struct.
/// rkyv_serialize_sample.rs
///
use rkyv::{deserialize, rancor::Error, Archive, Deserialize, Serialize};

/// This is the archived version of `TestStruct` and can be used in any context that requires a `TestStruct`.
/// It is a zero-copy deserialization of `TestStruct`. It can be used in any context that requires a `TestStruct`.
/// Allowing for zero-copy deserialization can be very useful in scenarios where memory efficiency is critical. For example, when working with large datasets or when transferring data over the network.
/// Archived types is `ArchiveTestStruct` struct.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
struct TestStruct {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

/// This use case demonstrates how to serialize a `TestStruct` using the `rkyv` crate.
/// The `rkyv` crate provides a zero-copy serialization of Rust types, which can be
/// very useful in scenarios where memory efficiency is critical.
/// For example, when working with large datasets or when transferring data over the network.
///  rkyv_serialize_sample.rs
fn rkyv_serialize_sample() {
    let value = TestStruct {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
    };

    // Serializing is as easy as a single function call
    let _bytes = rkyv::to_bytes::<Error>(&value).unwrap();

    // Or you can customize your serialization for better performance or control
    // over resource usage
    use rkyv::{api::high::to_bytes_with_alloc, ser::allocator::Arena};

    let mut arena = Arena::new();
    let bytes = to_bytes_with_alloc::<_, Error>(&value, arena.acquire()).unwrap();

    // You can use the safe API for fast zero-copy deserialization
    let archived = rkyv::access::<ArchivedTestStruct, Error>(&bytes[..]).unwrap();
    assert_eq!(archived, &value);

    // Or you can use the unsafe API for maximum performance
    let archived = unsafe { rkyv::access_unchecked::<ArchivedTestStruct>(&bytes[..]) };
    assert_eq!(archived, &value);

    // And you can always deserialize back to the original type
    let deserialized = deserialize::<TestStruct, Error>(archived).unwrap();
    assert_eq!(deserialized, value);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rkyv_serialize_sample() {
        rkyv_serialize_sample();
    }
}
