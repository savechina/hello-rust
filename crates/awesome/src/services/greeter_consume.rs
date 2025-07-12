mod helloworld {
    tonic::include_proto!("helloworld");
}

use anyhow::{anyhow, Context, Result};
use axum::http::StatusCode;
use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;
use rand::{thread_rng, Rng};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{oneshot, RwLock};
use tonic::{transport::Channel, Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
// use hyper::Server;
use tracing::{error, info, instrument, Level};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{filter::LevelFilter, EnvFilter}; // For `with_filter` and `EnvFilter` // For setting log level

use framework::{
    config::{BaseServiceConfig, ServiceConfig as TraitServiceConfig},
    error::FrameworkError,
    lifecycle::{RunnableService, ServiceStatus},
    registry::{AgentServiceCheck, AgentServiceRegistration, ConsulClient},
};

use crate::services::framework::{self, config::RegistryConfig};

// The main function for the client.
// #[tokio::main]
pub async fn start_consume(url: String) -> Result<()> {
    info!("Starting gRPC client for service discovery...");

    // --- 2. Configure Consul Client ---
    let consul_config = RegistryConfig {
        registry_url: "http://192.168.2.6:8500/v1/".to_string(),
        service_ip: "192.168.2.7".to_string(), // Client's IP, not strictly used for discovery, but good to have
    };
    let consul_client =
        ConsulClient::new(&consul_config.registry_url).context("Failed to create Consul client")?;

    let target_service_name = "my-greeter-service"; // The name our server registers with

    // --- 3. Service Discovery Loop ---
    let mut client_opt: Option<GreeterClient<Channel>> = None;
    let mut last_known_addr: Option<String> = None;
    let mut index = 0;
    let mut offset = 0;
    loop {
        // Query Consul for the service address
        let service_nodes = consul_client
            .discover_service(target_service_name)
            .await
            .context("Failed to query Consul for service")?;

        let count = service_nodes.len();

        if count == 0 {
            error!("No instances found for service '{}'", target_service_name);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            continue; // No instances found, wait and retry
        }
        // Update index and offset for round-robin discovery
        if offset >= count {
            offset = 0;
        }
        offset = offset + 1;

        index = offset % count;
        info!(
            "Service discovery iteration: {}, total instances: {}, current index: {}",
            offset, count, index
        );

        info!(
            "Discovered {} instances for service '{}'. fetching instance {}",
            service_nodes.len(),
            target_service_name,
            index
        );

        if let Some(service) = service_nodes.get(index) {
            // Use the discovered service node
            let addr = format!(
                "http://{}:{}",
                service.service_address, service.service_port
            );

            if last_known_addr.as_deref() != Some(&addr) {
                info!("Discovered new service address: {}", addr);
                last_known_addr = Some(addr.clone());

                // Create a new gRPC client with the discovered address
                match GreeterClient::connect(addr.clone()).await {
                    Ok(client) => {
                        client_opt = Some(client);
                        info!("Connected to gRPC server at {}", addr);
                    }
                    Err(e) => {
                        error!("Failed to connect to gRPC server: {}", e);
                    }
                }
            }
        } else {
            error!("Service '{}' not found in Consul", target_service_name);
        }

        // --- Make gRPC call if client is available ---
        if let Some(client) = &mut client_opt {
            // Use &mut client_opt to take mutable reference
            info!("Attempting gRPC call...");
            // Prepare the request
            let mut rng = thread_rng();
            let client_id = rng.gen_range(0..10000); // Random client ID for demonstration
                                                     // Create a HelloRequest with a name
                                                     // This is a simple request, you can modify it as needed.
                                                     // Create a HelloRequest with a name
            let request = HelloRequest {
                name: format!("Client#{}", client_id).into(),
            };
            // Use the client to make the gRPC call
            // Make the gRPC call
            match client.say_hello(request).await {
                Ok(response) => {
                    let msg = response.into_inner().message;
                    info!("gRPC Response: {}", msg);
                }
                Err(e) => {
                    error!("gRPC Call Failed: {}. Re-discovering...", e);
                    client_opt = None; // Invalidate client if call failed (e.g., connection lost)
                    last_known_addr = None;
                }
            }
        } else {
            info!("gRPC client not connected. Waiting for service discovery...");
        }

        // Sleep before the next discovery attempt
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    info!("Shutdown GreeterClient... done");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "tonic grpc server"]
    #[test]
    fn test_hello_client() {
        // --- 1. Initialize Logging ---
        // This should be done once at the very beginning of your application.
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

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_consume("http://192.168.2.7:50051".to_owned())
                .await
                .unwrap();
        });
    }
}
