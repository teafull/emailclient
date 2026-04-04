use crate::error::EmailError;
use crate::models::{Email, EmailDetail};
use chrono::Utc;

#[tauri::command]
pub async fn fetch_emails(
    email: String,
    password: String,
    folder: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<Email>, EmailError> {
    // Placeholder - actual IMAP fetch will be implemented
    // For now, return empty list
    Ok(Vec::new())
}

#[tauri::command]
pub async fn get_email_by_id(
    email: String,
    password: String,
    folder: String,
    uid: String,
) -> Result<EmailDetail, EmailError> {
    // Placeholder - actual email fetch will be implemented
    Err(EmailError::NotFound("Email not found".to_string()))
}

#[tauri::command]
pub async fn delete_email(
    email: String,
    password: String,
    folder: String,
    uid: String,
) -> Result<bool, EmailError> {
    // Placeholder - actual delete will be implemented
    Ok(true)
}

#[tauri::command]
pub async fn move_email(
    email: String,
    password: String,
    from_folder: String,
    to_folder: String,
    uid: String,
) -> Result<bool, EmailError> {
    // Placeholder - actual move will be implemented
    Ok(true)
}
