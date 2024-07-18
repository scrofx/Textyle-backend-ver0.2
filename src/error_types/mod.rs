pub mod login_error;
pub mod sign_up_error;
mod fetch_history_error;

// Re-export commonly used errors
pub use login_error::LoginError;
pub use sign_up_error::SignUpError;
pub use fetch_history_error::FetchHistoryError;