use crate::error::EmailError;
use crate::models::AttachmentData;

#[tauri::command]
pub async fn download_attachment(
    email: String,
    password: String,
    folder: String,
    uid: String,
    attachment_id: String,
) -> Result<AttachmentData, EmailError> {
    // Placeholder - actual attachment download will be implemented
    Err(EmailError::NotFound("Attachment not found".to_string()))
}
