use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
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
    pub fn new(registry_url: &str) -> Result<Self> {
        let base_url = Url::parse(registry_url).context("Invalid Consul API URL")?;
        Ok(Self {
            http_client: Client::new(),
            consul_api_base_url: base_url,
        })
    }

    #[instrument(name = "consul_register", skip(self, registration))]
    pub async fn register_service(&self, registration: &AgentServiceRegistration) -> Result<()> {
        // Ensure the registration has a valid ID
        if registration.id.is_none() {
            return Err(anyhow!("Service registration must have a valid ID"));
        }

        let reg_str = serde_json::to_string(&registration).unwrap();

        println!("Serialized AgentServiceRegistration: {}", reg_str);

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

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;
    use serde_json;

    #[test]
    fn test_registration_to_json() {
        let registration = AgentServiceRegistration {
            id: Some("test-service".to_string()),
            name: "Test Service".to_string(),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            address: Some("".to_string()),
            port: Some(8080),
            meta: Some(HashMap::new()),
            check: Some(AgentServiceCheck {
                check_id: Some("test-check".to_string()),
                name: Some("Test Check".to_string()),
                http: Some("http://localhost:8080/health".to_string()),
                tcp: None,
                interval: Some("10s".to_string()),
                timeout: Some("5s".to_string()),
                deregister_critical_service_after: Some("1m".to_string()),
            }),
        };
        // Serialize the registration to JSON
        // This will convert the AgentServiceRegistration struct into a JSON string
        // using serde_json's to_string function.
        let reg_str = serde_json::to_string(&registration).unwrap();

        println!("Serialized AgentServiceRegistration: {}", reg_str);
    }
}
