//! Error types for RsxUI.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RsxuiError {
    #[error("Invalid component name: {0}")]
    InvalidComponentName(String),

    #[error("Missing required prop: {0}")]
    MissingRequiredProp(String),

    #[error("Invalid attribute value: {0}")]
    InvalidAttributeValue(String),
}
