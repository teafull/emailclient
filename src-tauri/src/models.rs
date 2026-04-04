use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub uid: String,
    pub from: String,
    pub from_name: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub subject: String,
    pub preview: String,
    pub date: DateTime<Utc>,
    pub has_attachments: bool,
    pub is_read: bool,
    pub is_starred: bool,
    pub folder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDetail {
    pub email: Email,
    pub body_html: Option<String>,
    pub body_text: String,
    pub attachments: Vec<AttachmentInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentData {
    pub info: AttachmentInfo,
    pub data_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedAttachment {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub data_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub subject: String,
    pub body_html: String,
    pub body_text: String,
    pub attachments: Vec<PreparedAttachment>,
    pub signature_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpCredentials {
    pub email: String,
    pub password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub success: bool,
    pub server: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub message_id: String,
    pub sent_at: DateTime<Utc>,
}
