use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info, instrument, warn};
use url::Url;

// --- Consul API Structs (these are the actual data structures matching Consul's JSON API) ---

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AgentServiceRegistration {
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<AgentServiceCheck>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AgentServiceCheck {
    #[serde(skip_serializing_if = "Option::is_none", rename = "ID")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregister_critical_service_after: Option<String>,
    // Add other check types as needed (e.g., "TTL", "Script", "GRPC")
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CatalogServiceNode {
    pub node: String,
    pub address: String,
    pub service_id: String,
    pub service_name: String,
    pub service_address: String,
    pub service_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_meta: Option<HashMap<String, String>>,
}

/// A client for interacting with the Consul Agent and Catalog APIs.
#[derive(Debug, Clone)]
pub struct ConsulClient {
    http_client: Client,
    consul_api_base_url: Url,
}

impl ConsulClient {
    pub fn new(consul_api_url: &str) -> Result<Self> {
        let base_url = Url::parse(consul_api_url).context("Invalid Consul API URL")?;
        Ok(Self {
            http_client: Client::new(),
            consul_api_base_url: base_url,
        })
    }

    #[instrument(name = "consul_register", skip(self, registration))]
    pub async fn register_service(&self, registration: &AgentServiceRegistration) -> Result<()> {
        let url = self.consul_api_base_url.join("agent/service/register")?;

        let response = self
            .http_client
            .put(url)
            .json(registration)
            .send()
            .await
            .context("Failed to send Consul service registration request")?;

        let status = response.status();
        if status.is_success() {
            info!(
                "Successfully registered service '{}'. Status: {}",
                registration.id.as_deref().unwrap_or("N/A"),
                status
            );
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "N/A".to_string());
            Err(anyhow!(
                "Failed to register service '{}'. Status: {}, Body: {}",
                registration.id.as_deref().unwrap_or("N/A"),
                status,
                error_body
            ))
        }
    }

    #[instrument(name = "consul_deregister", skip(self))]
    pub async fn deregister_service(&self, service_id: &str) -> Result<()> {
        let url = self
            .consul_api_base_url
            .join(&format!("agent/service/deregister/{}", service_id))?;

        let response = self
            .http_client
            .put(url) // Deregister uses PUT with no body
            .send()
            .await
            .context("Failed to send Consul service deregistration request")?;

        let status = response.status();
        if status.is_success() {
            info!(
                "Successfully deregistered service '{}'. Status: {}",
                service_id, status
            );
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_else(|_| "N/A".to_string());
            warn!(
                "Failed to deregister service '{}'. Status: {}, Body: {}",
                service_id, status, error_body
            );
            Err(anyhow!(
                "Failed to deregister service '{}'. Status: {}, Body: {}",
                service_id,
                status,
                error_body
            ))
        }
    }

    #[instrument(name = "consul_discover", skip(self))]
    pub async fn discover_service(&self, service_name: &str) -> Result<Vec<CatalogServiceNode>> {
        let url = self
            .consul_api_base_url
            .join(&format!("catalog/service/{}", service_name))?;

        let nodes: Vec<CatalogServiceNode> = self
            .http_client
            .get(url)
            .send()
            .await
            .context(format!(
                "Failed to send service discovery request for '{}'",
                service_name
            ))?
            .json()
            .await
            .context(format!(
                "Failed to parse service discovery response for '{}'",
                service_name
            ))?;

        info!(
            "Discovered {} instances for service '{}'.",
            nodes.len(),
            service_name
        );
        Ok(nodes)
    }
}
