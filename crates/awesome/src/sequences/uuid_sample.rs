use uuid::Uuid;

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
}
