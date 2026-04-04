pub mod auth;
pub mod fetch;
pub mod send;
pub mod attachment;

pub use auth::verify_credentials;
pub use fetch::{fetch_emails, get_email_by_id, delete_email, move_email};
pub use send::send_email;
pub use attachment::download_attachment;
