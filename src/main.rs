//! Hello Rust — Interactive Learning CLI
//! 
//! A Rustlings-like CLI for learning Rust through runnable examples.
//! 
//! Usage:
//!   hello                    Show help and categories
//!   hello list               List all topics
//!   hello basic              List basic topics
//!   hello basic <topic>      Run a basic topic sample
//!   hello advance <topic>    Run an advance topic sample
//!   hello awesome <topic>    Run an awesome topic sample
//!   hello algo <topic>       Run an algo topic sample

#[macro_use]
extern crate cfg_if;
extern crate log;
extern crate getset;
extern crate inventory;

use clap::Parser;
use crate::cli::{Commands, dispatcher};

mod cli;
mod advance;
mod algo;
mod basic;
mod awesome_bridge;

/// Hello Rust CLI — Interactive Learning Tool
#[derive(Parser, Debug)]
#[command(name = "hello")]
#[command(author = "Hello Rust Contributors")]
#[command(version = "0.1.0")]
#[command(about = "Interactive Rustlings-like CLI for learning Rust", long_about = None)]
struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        // No subcommand: show help
        None => {
            dispatcher::list_all();
        }
        
        // List all topics
        Some(Commands::List) => {
            dispatcher::list_all();
        }
        
        // Basic category
        Some(Commands::Basic { topic, no_docs, force }) => {
            match topic {
                Some(topic_name) => {
                    // Run specific topic
                    if let Some(topic) = cli::registry::get_topic("basic", topic_name) {
                        if let Err(e) = dispatcher::execute_topic(topic, *no_docs, *force) {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    } else {
                        dispatcher::handle_unknown_topic("basic", topic_name);
                        std::process::exit(1);
                    }
                }
                None => {
                    // List all basic topics
                    dispatcher::list_category("basic");
                }
            }
        }
        
        // Advance category
        Some(Commands::Advance { topic, no_docs, force }) => {
            match topic {
                Some(topic_name) => {
                    if let Some(topic) = cli::registry::get_topic("advance", topic_name) {
                        if let Err(e) = dispatcher::execute_topic(topic, *no_docs, *force) {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    } else {
                        dispatcher::handle_unknown_topic("advance", topic_name);
                        std::process::exit(1);
                    }
                }
                None => {
                    dispatcher::list_category("advance");
                }
            }
        }
        
        // Awesome category
        Some(Commands::Awesome { topic, no_docs, force }) => {
            match topic {
                Some(topic_name) => {
                    if let Some(topic) = cli::registry::get_topic("awesome", topic_name) {
                        if let Err(e) = dispatcher::execute_topic(topic, *no_docs, *force) {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    } else {
                        dispatcher::handle_unknown_topic("awesome", topic_name);
                        std::process::exit(1);
                    }
                }
                None => {
                    dispatcher::list_category("awesome");
                }
            }
        }
        
        // Algo category
        Some(Commands::Algo { topic, no_docs, force }) => {
            match topic {
                Some(topic_name) => {
                    if let Some(topic) = cli::registry::get_topic("algo", topic_name) {
                        if let Err(e) = dispatcher::execute_topic(topic, *no_docs, *force) {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    } else {
                        dispatcher::handle_unknown_topic("algo", topic_name);
                        std::process::exit(1);
                    }
                }
                None => {
                    dispatcher::list_category("algo");
                }
            }
        }
    }
}
