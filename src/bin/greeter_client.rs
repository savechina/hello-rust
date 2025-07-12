use anyhow::Result; // Use anyhow for top-level main function error handling
use awesome::services::framework::{
    config::{BaseServiceConfig, RegistryConfig},
    lifecycle::{ApplicationFramework, RunnableService, ServiceStatus},
};
use awesome::services::greeter_service::{GreeterApplicationService, GreeterServiceConfig};
use tokio::time::{sleep, Duration};
use tracing::{error, info, span, Level};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{filter::LevelFilter, EnvFilter}; // For `with_filter` and `EnvFilter` // For setting log level
                                                          // use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the tracing subscriber
    // This should be done once at the very beginning of your application.
    tracing_subscriber::registry()
        .with(
            fmt::layer() // Use the fmt layer for console output
                .compact() // Make the output more compact (optional)
                .with_target(true) // Include the target (module path) of the event
                .with_level(true) // Include the log level
                .with_thread_ids(true) // Include thread IDs (optional)
                .with_thread_names(true), // Include thread names (optional)
        )
        .with(
            EnvFilter::from_default_env() // Allow filtering via RUST_LOG env var
                .add_directive(Level::INFO.into()), // Default log level if RUST_LOG is not set
        )
        .init(); // Initialize the global default subscriber
    let _span_ = span!(Level::TRACE, "greeter_client_startup").entered();
    info!("Starting application framework...");
    // --- 2. Define Base Configuration ---
    // In a real app, you might load this from a file (e.g., JSON, YAML)
    let base_config = BaseServiceConfig {
        service_id_prefix: "greeter-app".to_string(),
        service_name: "my-greeter-service".to_string(),
        consul: RegistryConfig {
            registry_url: "http://192.168.2.6:8500/v1/".to_string(),
            service_ip: "192.168.2.7".to_string(),
        },
    };
    // --- 3. Define Greeter Service Specific Configuration ---

    Ok(())
}
