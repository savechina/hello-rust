use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use axum::http::StatusCode;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{oneshot, RwLock};
use tonic::{Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
// use hyper::Server;
use tracing::{error, info, instrument};

use framework::{
    config::{BaseServiceConfig, ServiceConfig as TraitServiceConfig},
    error::FrameworkError,
    lifecycle::{RunnableService, ServiceStatus},
    registry::{AgentServiceCheck, AgentServiceRegistration, ConsulClient},
};

// --- gRPC auto-generated code ---
pub mod helloworld {
    tonic::include_proto!("helloworld");
}
use helloworld::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

use crate::services::framework;

// --- gRPC Service Logic ---
#[derive(Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[instrument(name = "say_hello", skip(self, request))]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        info!("Received greeting request from: {}", name);
        let reply = helloworld::HelloReply {
            message: format!("Hello {} from gRPC Greeter Service!", name),
        };
        Ok(Response::new(reply))
    }
}

// --- Greeter Service Specific Configuration ---
#[derive(Debug, Clone)]
pub struct GreeterServiceConfig {
    pub base_config: BaseServiceConfig,
    pub grpc_port: u16,
    pub http_health_port: u16,
}

impl TraitServiceConfig for GreeterServiceConfig {
    fn base_config(&self) -> &BaseServiceConfig {
        &self.base_config
    }
}

// --- Greeter Service Implementation of RunnableService ---
#[derive(Debug)]
pub struct GreeterApplicationService {
    config: GreeterServiceConfig,
    instance_id: String,
    status: Arc<RwLock<ServiceStatus>>,
    consul_client: ConsulClient,
}

#[async_trait]
impl RunnableService for GreeterApplicationService {
    type Config = GreeterServiceConfig;

    fn new(
        config: Self::Config,
        instance_id: String,
        status_arc: Arc<RwLock<ServiceStatus>>,
    ) -> Self {
        let consul_client = ConsulClient::new(&config.base_config.consul.registry_url)
            .expect("Failed to create Consul client in GreeterService");
        Self {
            config,
            instance_id,
            status: status_arc,
            consul_client,
        }
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn get_status_arc(&self) -> Arc<RwLock<ServiceStatus>> {
        Arc::clone(&self.status)
    }

    #[instrument(name = "greeter_service_logic", skip(self, shutdown_rx))]
    async fn start_service_logic(
        &self,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), FrameworkError> {
        let grpc_addr: SocketAddr = format!(
            "{}:{}",
            self.config.base_config.consul.service_ip, self.config.grpc_port
        )
        .parse()
        .map_err(|e| FrameworkError::Config(format!("Invalid gRPC bind address: {}", e)))?;
        let http_health_addr: SocketAddr = format!(
            "{}:{}",
            self.config.base_config.consul.service_ip, self.config.http_health_port
        )
        .parse()
        .map_err(|e| FrameworkError::Config(format!("Invalid HTTP health bind address: {}", e)))?;

        // --- Build gRPC Service ---
        let greeter_service = helloworld::greeter_server::GreeterServer::new(MyGreeter::default());

        let grpc_server = tonic::transport::Server::builder()
            .layer(TraceLayer::new_for_grpc()) // Add gRPC tracing (from tower-http)
            // Add other Tower middleware here, e.g., .timeout(), .rate_limit()
            .add_service(greeter_service)
            .serve(grpc_addr);

        // --- Start HTTP Health Check Server ---
        let health_app: axum::Router<()> = axum::Router::new().route(
            "/health",
            axum::routing::get(|| async {
                // In a real application, perform internal health checks here (e.g., database connection)
                StatusCode::OK
            }),
        );

        let listener = tokio::net::TcpListener::bind(&http_health_addr)
            .await
            .unwrap();

        let health_server = axum::serve(listener, health_app);

        // let health_server = Server::bind(&http_health_addr).serve(health_app.into_make_service());

        // --- Register with Consul ---
        let registration_payload = AgentServiceRegistration {
            id: Some(self.instance_id.clone()),
            name: self.config.base_config.service_name.clone(),
            tags: Some(vec!["grpc".to_string(), "greeter".to_string()]),
            address: Some(self.config.base_config.consul.service_ip.clone()),
            port: Some(self.config.grpc_port),
            meta: Some({
                let mut meta = HashMap::new();
                meta.insert("protocol".to_string(), "grpc".to_string());
                meta.insert("version".to_string(), "1.0.0".to_string());
                meta
            }),
            check: Some(AgentServiceCheck {
                check_id: Some(format!("{}-health", self.instance_id)),
                name: Some("HTTP Health Check".to_string()),
                http: Some(format!(
                    "http://{}:{}/health",
                    self.config.base_config.consul.service_ip, self.config.http_health_port
                )),
                interval: Some("10s".to_string()),
                timeout: Some("1s".to_string()),
                deregister_critical_service_after: Some("1m".to_string()),
                tcp: None, // No TCP check for now
            }),
        };

        self.consul_client
            .register_service(&registration_payload)
            .await
            .map_err(|e| {
                FrameworkError::Consul(format!("Failed to register service with Consul: {}", e))
            })?;

        // --- Wait for Shutdown or Internal Server Failure ---
        tokio::select! {
            grpc_res = grpc_server => {
                if let Err(e) = grpc_res {
                    error!("gRPC server for '{}' stopped with error: {}", self.instance_id, e);
                    return Err(FrameworkError::Startup(format!("gRPC server failed: {}", e)));
                } else {
                    info!("gRPC server for '{}' stopped gracefully.", self.instance_id);
                }
            }
            health_res = health_server => {
                if let Err(e) = health_res {
                    error!("Health check server for '{}' stopped with error: {}", self.instance_id, e);
                    return Err(FrameworkError::Startup(format!("Health check server failed: {}", e)));
                } else {
                    info!("Health check server for '{}' stopped gracefully.", self.instance_id);
                }
            }
            _ = shutdown_rx => {
                info!("Shutdown signal received for '{}'.", self.instance_id);
            }
        }

        // --- Deregister from Consul ---
        self.consul_client
            .deregister_service(&self.instance_id)
            .await
            .map_err(|e| {
                FrameworkError::Consul(format!("Failed to deregister service from Consul: {}", e))
            })?;

        Ok(())
    }
}
