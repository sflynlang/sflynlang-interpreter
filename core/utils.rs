pub mod restricted_names;

pub fn get_current_directory() -> String {
    if let Ok(current_directory) = std::env::current_dir() {
        current_directory.display().to_string()
    } else {
        String::new()
    }
}
