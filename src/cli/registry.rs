//! Sample registry using inventory crate for compile-time automatic registration
//!
//! New samples are registered by adding `inventory::submit!` declarations.
//! No manual routing table updates required.

use inventory::{collect, iter};

/// Type of sample (for execution handling)
#[derive(Debug, Clone, Copy)]
pub enum SampleType {
    /// Synchronous or async function that runs and returns
    Function,
    /// Long-running server process (requires separate terminal)
    Server,
}

/// A Topic represents a single learnable unit with its sample function
#[derive(Debug, Clone)]
pub struct Topic {
    /// CLI subcommand identifier (kebab-case, e.g. "ownership", "smart-pointers")
    pub name: &'static str,

    /// One-line Chinese description with English technical terms
    pub description: &'static str,

    /// Parent category: "basic", "advance", "awesome", "algo"
    pub category: &'static str,

    /// Zero-argument function pointer to execute the sample
    pub func: fn(),

    /// External services required (empty slice if none)
    /// Valid values: "Consul", "MySQL", "SQLite", "MQTT", "Ollama", "gRPC"
    pub dependencies: &'static [&'static str],

    /// Path to embedded markdown tutorial (e.g., "basic/ownership.md")
    pub doc_path: Option<&'static str>,

    /// Whether this is a function or server sample
    pub sample_type: SampleType,
}

// Register the inventory collection for all topics
collect!(Topic);

impl Topic {
    /// Create a new Topic instance
    pub const fn new(
        name: &'static str,
        description: &'static str,
        category: &'static str,
        func: fn(),
        dependencies: &'static [&'static str],
        doc_path: Option<&'static str>,
        sample_type: SampleType,
    ) -> Self {
        Self {
            name,
            description,
            category,
            func,
            dependencies,
            doc_path,
            sample_type,
        }
    }

    /// Check if this topic has external dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }
}

/// Get all registered topics
pub fn get_all_topics() -> impl Iterator<Item = &'static Topic> {
    iter::<Topic>()
}

/// Get topics filtered by category
pub fn get_topics_by_category(category: &str) -> impl Iterator<Item = &'static Topic> {
    iter::<Topic>().filter(move |topic| topic.category == category)
}

/// Get a specific topic by category and name
pub fn get_topic(category: &str, name: &str) -> Option<&'static Topic> {
    iter::<Topic>().find(|topic| topic.category == category && topic.name == name)
}

/// Get all unique categories
pub fn get_all_categories() -> impl Iterator<Item = &'static str> {
    let mut categories = Vec::new();
    for topic in iter::<Topic>() {
        if !categories.contains(&topic.category) {
            categories.push(topic.category);
        }
    }
    categories.into_iter()
}
