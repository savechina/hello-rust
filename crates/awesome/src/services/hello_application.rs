use anyhow::Result; // Use anyhow for top-level main function error handling
use framework::{
    config::{BaseServiceConfig, ConsulConfig},
    lifecycle::{ApplicationFramework, ServiceStatus},
};
use my_app_framework::services::greeter::{GreeterApplicationService, GreeterServiceConfig};
use tokio::time::{sleep, Duration};
use tracing::{error, info, Level};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // --- 1. Initialize Logging ---
    fmt::subscriber()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    info!("Starting application framework...");

    // --- 2. Define Base Configuration ---
    // In a real app, you might load this from a file (e.g., JSON, YAML)
    let base_config = BaseServiceConfig {
        service_id_prefix: "greeter-app".to_string(),
        service_name: "my-greeter-service".to_string(),
        consul: ConsulConfig {
            api_url: "http://127.0.0.1:8500/v1".to_string(),
            service_ip: "127.0.0.1".to_string(),
        },
    };

    // --- 3. Define Greeter Service Specific Configuration ---
    let greeter_config = GreeterServiceConfig {
        base_config,
        grpc_port: 50051,
        http_health_port: 8080,
    };

    // --- 4. Initialize and Start the Service via the Framework ---
    let app_framework = ApplicationFramework::new(greeter_config)
        .expect("Failed to initialize ApplicationFramework");

    info!(
        "Application service '{}' started in background.",
        app_framework.service().instance_id()
    );

    // --- 5. Main Application Loop (Simulates ongoing work and status monitoring) ---
    for i in 0..10 {
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
