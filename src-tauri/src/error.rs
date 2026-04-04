use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum EmailError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("IMAP connection error: {0}")]
    ImapError(String),

    #[error("SMTP connection error: {0}")]
    SmtpError(String),

    #[error("Email not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Attachment error: {0}")]
    AttachmentError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}
