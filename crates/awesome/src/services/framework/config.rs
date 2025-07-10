use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsulConfig {
    pub api_url: String,
    pub service_ip: String,
}

impl Default for ConsulConfig {
    fn default() -> Self {
        Self {
            api_url: "http://127.0.0.1:8500/v1/".to_string(),
            service_ip: "127.0.0.1".to_string(),
        }
    }
}

// Base configuration common to all services managed by the framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseServiceConfig {
    pub service_id_prefix: String,
    pub service_name: String,
    pub consul: ConsulConfig,
    // Add other common config here, e.g., logging levels, metrics endpoints
}

impl Default for BaseServiceConfig {
    fn default() -> Self {
        Self {
            service_id_prefix: "default-service".to_string(),
            service_name: "default-app-service".to_string(),
            consul: ConsulConfig::default(),
        }
    }
}

/// A trait for any service-specific configuration.
/// This allows each `RunnableService` to define its own config type.
pub trait ServiceConfig: Debug + Clone + Send + Sync + 'static {
    fn base_config(&self) -> &BaseServiceConfig;
}
