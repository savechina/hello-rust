use anyhow::Result; // Use anyhow for top-level main function error handling
use awesome::services::framework::{
    config::{BaseServiceConfig, ConsulConfig},
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

    let _span_ = span!(Level::TRACE, "app_startup").entered();
    info!("Starting application framework...");

    // --- 2. Define Base Configuration ---
    // In a real app, you might load this from a file (e.g., JSON, YAML)
    let base_config = BaseServiceConfig {
        service_id_prefix: "greeter-app".to_string(),
        service_name: "my-greeter-service".to_string(),
        consul: ConsulConfig {
            api_url: "http://192.168.2.6:8500/v1/".to_string(),
            service_ip: "192.168.2.7".to_string(),
        },
    };

    // --- 3. Define Greeter Service Specific Configuration ---
    let greeter_config = GreeterServiceConfig {
        base_config,
        grpc_port: 50051,
        http_health_port: 8080,
    };

    // --- 4. Initialize and Start the Service via the Framework ---
    let mut app_framework = ApplicationFramework::<GreeterApplicationService>::new(greeter_config)
        .expect("Failed to initialize ApplicationFramework");

    info!(
        "Application service '{}' started in background.",
        app_framework.service().instance_id()
    );

    // --- 5. Main Application Loop (Simulates ongoing work and status monitoring) ---
    for i in 0..100 {
        sleep(Duration::from_secs(3)).await;
        let current_status = app_framework.get_status().await;
        info!(
            "Main loop ({}s): Current service status: {:?}",
            i * 3,
            current_status
        );

        if matches!(current_status, ServiceStatus::Failed(_)) {
            error!("Service entered a failed state. Initiating immediate exit.");
            break; // Service failed, exit loop
        }
    }

    // --- 6. Handle OS Signals for Graceful Shutdown ---
    info!("Press Ctrl+C to initiate graceful shutdown...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    info!("Ctrl+C received. Initiating graceful shutdown via framework.");

    // --- 7. Stop the Service Gracefully ---
    match app_framework.stop().await {
        Ok(_) => info!("Application service gracefully stopped."),
        Err(e) => error!("Application service stopped with error: {}", e),
    }

    // --- 8. Final Status Check ---
    let final_status = app_framework.get_status().await;
    info!("Final service status: {:?}", final_status);

    Ok(())
}
