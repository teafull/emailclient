use chrono::Utc;
use crate::error::EmailError;
use crate::models::{SendEmailRequest, SendResult, SmtpCredentials};

#[tauri::command]
pub async fn send_email(
    credentials: SmtpCredentials,
    message: SendEmailRequest,
) -> Result<SendResult, EmailError> {
    // Placeholder - actual SMTP send will be implemented
    // For now, just return a mock success
    Ok(SendResult {
        message_id: uuid::Uuid::new_v4().to_string(),
        sent_at: Utc::now(),
    })
}
