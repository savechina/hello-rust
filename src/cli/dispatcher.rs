//! CLI dispatcher and execution engine
//!
//! Routes subcommands to sample functions and handles execution

use crate::cli::docs_embed;
use crate::cli::{registry, Commands};

/// Execute a topic's sample function
///
/// # Arguments
/// * `topic` - The topic to execute
/// * `no_docs` - Whether to skip tutorial display
/// * `force` - Whether to skip dependency checks
pub fn execute_topic(topic: &registry::Topic, no_docs: bool, force: bool) -> Result<(), String> {
    // Check dependencies unless --force is set
    if !force && topic.has_dependencies() {
        eprintln!(
            "⚠ Warning: Topic '{}' requires external services: {}.",
            topic.name,
            topic.dependencies.join(", ")
        );
        eprintln!("   These services may not be available.");
        eprintln!();
        eprintln!("   Run with --force to skip this check.");
        eprintln!();
    }

    // Check if this is a server-type sample
    match topic.sample_type {
        registry::SampleType::Server => {
            eprintln!("⚠ Note: Topic '{}' is a long-running server.", topic.name);
            eprintln!("   It requires a separate terminal.");
            eprintln!();
        }
        registry::SampleType::Function => {
            // Display tutorial content unless --no-docs is set
            if !no_docs {
                if let Some(doc_path) = topic.doc_path {
                    if docs_embed::display_tutorial(doc_path) {
                        println!();
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                        println!("▶ Running sample:");
                        println!();
                    }
                }
            }

            // Execute the sample function
            (topic.func)();
        }
    }

    Ok(())
}

/// List all topics in a category
///
/// # Arguments
/// * `category` - Category name to list
pub fn list_category(category: &str) {
    let topics: Vec<_> = registry::get_topics_by_category(category).collect();

    if topics.is_empty() {
        eprintln!("No topics found in category '{}'.", category);
        return;
    }

    println!();
    println!(
        "{} topics — Run: hello {} <topic>",
        get_category_display_name(category),
        category
    );
    println!();
    println!("{:<25}  {:<45}  {}", "TOPIC", "DESCRIPTION", "DEPENDENCIES");
    println!("{:-<25}  {:-<45}  {:-<15}", "", "", "");

    for topic in topics {
        let deps = if topic.dependencies.is_empty() {
            "无".to_string()
        } else {
            topic.dependencies.join(", ")
        };
        println!("{:<25}  {:<45}  {}", topic.name, topic.description, deps);
    }
    println!();
}

/// List all categories and topics
pub fn list_all() {
    println!();
    println!("Hello Rust — Interactive Learning CLI");
    println!();
    println!("Categories:");
    println!();

    for category in registry::get_all_categories() {
        let topic_count = registry::get_topics_by_category(category).count();
        println!("  {:<12}  {} topics", format!("{}:", category), topic_count);
    }
    println!();
    println!("Run `hello <category> --help` to list topics in a category.");
    println!("Run `hello <category> <topic>` to run a specific sample.");
    println!();
}

/// Get display name for a category
fn get_category_display_name(category: &str) -> String {
    match category {
        "basic" => "Rust 基础 (Basic)".to_string(),
        "advance" => "Rust 进阶 (Advance)".to_string(),
        "awesome" => "生产级样例 (Awesome)".to_string(),
        "algo" => "算法 (Algorithms)".to_string(),
        _ => category.to_string(),
    }
}

/// Handle unknown topic with suggestions
///
/// # Arguments
/// * `category` - The category that was searched
/// * `topic` - The topic name that was not found
pub fn handle_unknown_topic(category: &str, topic: &str) {
    let available: Vec<_> = registry::get_topics_by_category(category)
        .map(|t| t.name)
        .collect();

    eprintln!();
    eprintln!("Error: Unknown topic '{}' in '{}'.", topic, category);
    eprintln!();
    eprintln!("Available topics:");
    for name in available {
        // Simple suggestion: show all available topics
        // Could implement fuzzy matching here
        eprintln!("  {}", name);
    }
    eprintln!();
}

/// Handle unknown category
///
/// # Arguments
/// * `category` - The category name that was not found
pub fn handle_unknown_category(category: &str) {
    eprintln!();
    eprintln!("Error: Unknown category '{}'.", category);
    eprintln!();
    eprintln!("Available categories:");
    for cat in registry::get_all_categories() {
        eprintln!("  {}", cat);
    }
    eprintln!();
}
