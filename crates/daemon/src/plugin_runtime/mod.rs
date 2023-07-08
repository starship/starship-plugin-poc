mod bindings;
pub use self::bindings::*;

mod types;
pub use self::types::*;

fn current_dir() -> String {
    std::env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert current directory to string")
        .to_string()
}
