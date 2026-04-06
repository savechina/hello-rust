//! CLI module for interactive hello-rust subcommands
//!
//! Provides clap-based subcommand structure for running individual samples

use clap::Subcommand;

pub mod dispatcher;
pub mod docs_embed;
pub mod registry;

/// Top-level CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available topics across all categories
    List,

    /// Rust fundamentals (ownership, generics, threads, ...)
    Basic {
        /// Specific topic to run (omit to list all)
        topic: Option<String>,

        /// Skip tutorial documentation and run sample directly
        #[arg(long)]
        no_docs: bool,

        /// Skip external dependency checks
        #[arg(long)]
        force: bool,
    },

    /// Advanced patterns (async, databases, web, ...)
    Advance {
        /// Specific topic to run (omit to list all)
        topic: Option<String>,

        /// Skip tutorial documentation and run sample directly
        #[arg(long)]
        no_docs: bool,

        /// Skip external dependency checks
        #[arg(long)]
        force: bool,
    },

    /// Production-grade samples (gRPC, DI, templates, ...)
    Awesome {
        /// Specific topic to run (omit to list all)
        topic: Option<String>,

        /// Skip tutorial documentation and run sample directly
        #[arg(long)]
        no_docs: bool,

        /// Skip external dependency checks
        #[arg(long)]
        force: bool,
    },

    /// Algorithm implementations (linked-list, pi calculation, ...)
    Algo {
        /// Specific topic to run (omit to list all)
        topic: Option<String>,

        /// Skip tutorial documentation and run sample directly
        #[arg(long)]
        no_docs: bool,

        /// Skip external dependency checks
        #[arg(long)]
        force: bool,
    },
}

impl Commands {
    /// Get the category name as a string
    pub fn category_name(&self) -> Option<&'static str> {
        match self {
            Commands::List => None,
            Commands::Basic { .. } => Some("basic"),
            Commands::Advance { .. } => Some("advance"),
            Commands::Awesome { .. } => Some("awesome"),
            Commands::Algo { .. } => Some("algo"),
        }
    }

    /// Get the topic if specified
    pub fn topic(&self) -> Option<&String> {
        match self {
            Commands::List => None,
            Commands::Basic { topic, .. } => topic.as_ref(),
            Commands::Advance { topic, .. } => topic.as_ref(),
            Commands::Awesome { topic, .. } => topic.as_ref(),
            Commands::Algo { topic, .. } => topic.as_ref(),
        }
    }

    /// Check if no_docs flag is set
    pub fn no_docs(&self) -> bool {
        match self {
            Commands::List => false,
            Commands::Basic { no_docs, .. } => *no_docs,
            Commands::Advance { no_docs, .. } => *no_docs,
            Commands::Awesome { no_docs, .. } => *no_docs,
            Commands::Algo { no_docs, .. } => *no_docs,
        }
    }

    /// Check if force flag is set
    pub fn force(&self) -> bool {
        match self {
            Commands::List => false,
            Commands::Basic { force, .. } => *force,
            Commands::Advance { force, .. } => *force,
            Commands::Awesome { force, .. } => *force,
            Commands::Algo { force, .. } => *force,
        }
    }
}
