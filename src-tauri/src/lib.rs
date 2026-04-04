mod commands;
mod error;
mod models;

use commands::auth::verify_credentials;
use commands::fetch::{delete_email, fetch_emails, get_email_by_id, move_email};
use commands::send::send_email;
use commands::attachment::download_attachment;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            verify_credentials,
            fetch_emails,
            get_email_by_id,
            delete_email,
            move_email,
            send_email,
            download_attachment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
