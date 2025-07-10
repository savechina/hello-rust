use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Configuration for the Consul service registry
/// This struct defines the necessary parameters to connect to a Consul instance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegistryConfig {
    // URL of the Consul registry API
    // This should point to the base URL of the Consul API, e.g., "http://127.0.0.1:8500/v1/"
    pub registry_url: String,
    // The IP address of the service instance
    // This is used to register the service with Consul and can be overridden by the service
    // if it needs to bind to a different IP (e.g., in containerized environments)
    // Default is "127.0.0.1" for local development
    pub service_ip: String,
}

/// Default implementation for `RegistryConfig`
/// This provides sensible defaults for local development and testing
/// The default registry URL points to a local Consul instance, and the service IP is set
impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            registry_url: "http://127.0.0.1:8500/v1/".to_string(),
            service_ip: "127.0.0.1".to_string(),
        }
    }
}

// Base configuration common to all services managed by the framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseServiceConfig {
    pub service_id_prefix: String,
    pub service_name: String,
    pub consul: RegistryConfig,
    // Add other common config here, e.g., logging levels, metrics endpoints
}

impl Default for BaseServiceConfig {
    fn default() -> Self {
        Self {
            service_id_prefix: "default-service".to_string(),
            service_name: "default-app-service".to_string(),
            consul: RegistryConfig::default(),
        }
    }
}

/// A trait for any service-specific configuration.
/// This allows each `RunnableService` to define its own config type.
pub trait ServiceConfig: Debug + Clone + Send + Sync + 'static {
    fn base_config(&self) -> &BaseServiceConfig;
}
