use thiserror::Error;

#[derive(Debug, Error)]
pub enum PortRecommenderError {
    #[error("name cannot be empty")]
    EmptyName,
    #[error("failed to inspect local ports: {0}")]
    PortInspection(String),
    #[error("no available port found in range 1024-65535")]
    NoAvailablePort,
}
