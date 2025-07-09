use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Service initialization failed: {0}")]
    Initialization(String),
    #[error("Service startup failed: {0}")]
    Startup(String),
    #[error("Service shutdown failed: {0}")]
    Shutdown(String),
    #[error("Consul interaction failed: {0}")]
    Consul(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Internal framework error: {0}")]
    Internal(String),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error), // Catch-all for other errors
}
