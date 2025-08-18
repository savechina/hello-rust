use std::{io::Write, time::SystemTime};

use chrono::Local;
use uuid::{NoContext, Timestamp, Uuid};

fn uuid_sample() {
    // 1. Generate a Version 4 (random) UUID
    // This is the most common type for general-purpose unique identifiers.
    let uuid_v4 = Uuid::new_v4();
    println!("Version 4 UUID: {}", uuid_v4); // Example: 936c342f-76a0-4a8b-8e1d-3b7c8a9e0f1d

    // 2. Generate a NIL UUID (all zeros)
    // Represents an "empty" or "null" UUID.
    let uuid_nil = Uuid::nil();
    println!("NIL UUID:       {}", uuid_nil); // Output: 00000000-0000-0000-0000-000000000000

    // 3. Generate a V3 (name-based, MD5 hash) or V5 (name-based, SHA-1 hash) UUID
    // These are generated from a namespace UUID and a name string, ensuring the same input always produces the same UUID.
    // First, define a namespace UUID (you can use a well-known one or generate your own).
    let namespace_url = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(); // RFC 4122 for URL namespace

    println!("Namespace UUID: {}", namespace_url); // Example: 6ba7b810-9dad-11d1-80b4-00c04fd430c8

    let name = "example.com";
    let uuid_v3 = Uuid::new_v3(&namespace_url, name.as_bytes());
    println!("Version 3 UUID: {}", uuid_v3); // Example: a5607a7c-4734-39fe-921c-a96c1664e43f

    let uuid_v5 = Uuid::new_v5(&namespace_url, name.as_bytes());
    println!("Version 5 UUID: {}", uuid_v5); // Example: c0d2a9f2-2b6d-5b3a-8a5e-b8d2b9f3a6a9

    // 4. Generate a URN (Uniform Resource Name) representation
    println!("URN representation: {}", uuid_v4.urn()); // Example: urn:uuid:936c342f-76a0-4a8b-8e1d-3b7c8a9e0f1d

    // You can also parse UUIDs from strings
    let uuid_str = "f8a7e0d1-c2b3-4a5b-6c7d-8e9f0a1b2c3d";
    match Uuid::parse_str(uuid_str) {
        Ok(parsed_uuid) => println!("Parsed UUID: {}", parsed_uuid),
        Err(e) => println!("Failed to parse UUID: {}", e),
    }

    // 5. Generate a v7 UUID

    let uuid_v7 = Uuid::now_v7();
    println!("Generated UUID v7: {}", uuid_v7);

    //current unixtime
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let now_sec = now.as_secs();

    let ns = now.subsec_nanos();

    let ts = Timestamp::from_unix(NoContext, now_sec, ns);

    let uuid_v7_2 = Uuid::new_v7(ts);

    println!("Generated UUID v7: {}", uuid_v7_2);

    let ts = Timestamp::from_unix_time(now_sec, ns, 123456, 14);

    let uuid_v7_3 = Uuid::new_v7(ts);

    println!("Generated UUID v7: {}", uuid_v7_3)
}

// use md5::{Digest, Md5};

/// Generates a GUID (UUID) from the MD5 hash of multiple fields.
///
/// This function concatenates all provided fields, computes their MD5 hash,
/// and then uses the resulting 16 bytes to create a UUID.
///
/// # Arguments
/// * `tenant_id` - The tenant ID.
/// * `area_id` - The area ID.
/// * `area_code` - The area code.
/// * `object_code` - The object code.
/// * `object_type` - The object type.
///
/// # Returns
/// A `uuid::Uuid` generated from the MD5 hash.
pub fn generate_guid_from_fields(
    tenant_id: &str,
    area_id: u64,
    area_code: &str,
    object_code: &str,
    object_type: i32,
) -> Uuid {
    // 1. Combine all fields into a single string.
    // The order of concatenation is important for consistent results.
    let combined_context = format!(
        "{}-{}-{}-{}-{}",
        tenant_id, area_id, area_code, object_code, object_type
    );

    // 2. Compute the MD5 hash of the combined string.
    let mut hasher = md5::Context::new();

    hasher.write(combined_context.as_bytes());
    let result = hasher.finalize();

    let md5_bytes = result.into();

    // 3. Create a UUID from the 16-byte MD5 hash.
    Uuid::from_bytes(md5_bytes)
}

fn md5_sample() {
    let tenant_id = "T001";
    let area_id = 1001;
    let area_code = "A01";
    let object_code = "OBJ-001";
    let object_type = 0;

    let guid = generate_guid_from_fields(tenant_id, area_id, area_code, object_code, object_type);

    println!("Generated GUID: {}", guid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    #[test]
    fn test_generate_uuid() {
        let uuid = Uuid::new_v4();
        assert!(uuid.is_nil() == false, "Generated UUID should not be nil");
        println!("Generated UUID: {}", uuid);

        uuid_sample();
    }

    #[test]
    fn test_generate_md5_uuid() {
        let uuid = Uuid::new_v4();
        assert!(uuid.is_nil() == false, "Generated UUID should not be nil");
        println!("Generated UUID: {}", uuid);

        md5_sample();
    }
}
