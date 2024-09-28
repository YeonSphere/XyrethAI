use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeokjinError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Sensitive information detected: {0}")]
    SensitiveInformation(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Data preprocessing error: {0}")]
    DataPreprocessingError(String),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Learning module error: {0}")]
    LearningModuleError(String),
    #[error("Interaction error: {0}")]
    InteractionError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
    #[error("Voice module error: {0}")]
    VoiceModuleError(String),
}

impl From<std::io::Error> for SeokjinError {
    fn from(error: std::io::Error) -> Self {
        SeokjinError::IOError(error.to_string())
    }
}

impl From<serde_json::Error> for SeokjinError {
    fn from(error: serde_json::Error) -> Self {
        SeokjinError::SerializationError(error.to_string())
    }
}

impl From<reqwest::Error> for SeokjinError {
    fn from(error: reqwest::Error) -> Self {
        SeokjinError::NetworkError(error.to_string())
    }
}
