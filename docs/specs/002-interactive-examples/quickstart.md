# Quickstart: Interactive CLI Subcommands

## For Users

### Running Samples

```bash
# Build the project
cargo build

# See all available commands
cargo run -- --help

# List all topics across categories
cargo run -- list

# Run a specific basic sample
cargo run -- basic ownership

# Run a specific advance sample
cargo run -- advance async

# Run a sample without tutorial content
cargo run -- basic ownership --no-docs

# Skip dependency checks
cargo run -- awesome grpc --force
```

## For Contributors: Adding a New Sample

### Step 1: Create the sample file

```rust
// src/basic/my_topic_sample.rs

/// Demonstrates my topic concept
pub fn my_topic_sample() {
    println!("Hello from my topic!");
    // Your sample code here
}
```

### Step 2: Register in mod.rs

```rust
// src/basic/mod.rs

pub mod my_topic_sample;

use crate::cli::registry::Topic;

inventory::submit! {
    Topic {
        name: "my-topic",
        description: "演示我的主题概念",
        category: "basic",
        func: my_topic_sample::my_topic_sample,
        dependencies: &[],
        doc_path: Some("basic/my-topic.md"),
        sample_type: crate::cli::registry::SampleType::Function,
    }
}
```

### Step 3: Add tutorial content (optional)

Create `docs/src/basic/my-topic.md`. It will be automatically embedded via `include_dir!`.

### Step 4: Verify

```bash
cargo build
cargo run -- basic my-topic
```

## For Contributors: Adding a New Category

1. Create `src/<category>/` directory with sample files
2. Create `src/<category>.rs` as the module root
3. Add `inventory::submit!` declarations for each topic
4. Add the category variant to `src/cli/mod.rs` Commands enum
5. Update the help text in `main.rs`
