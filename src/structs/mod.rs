pub mod user_struct;
pub mod login_form;
pub mod user_history;
pub mod claims;
pub mod data_prompt;
pub mod ml_result;


// Re-export commonly used structs
pub use user_struct::User;
pub use login_form::LoginForm;
pub use data_prompt::DataPrompt;
pub use claims::Claims;
pub use ml_result::MlRequestResult;
pub use user_history::UserHistory;