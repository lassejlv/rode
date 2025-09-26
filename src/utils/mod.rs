pub mod console;
pub mod fs;
pub mod http;
pub mod password;
pub mod path;
pub mod uuid;

pub use console::setup_console;
pub use fs::setup_fs;
pub use http::setup_http;
pub use password::setup_password;
pub use path::setup_path;
pub use uuid::setup_uuid;
