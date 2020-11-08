pub mod restricted_names;

pub fn get_current_directory() -> String {
    if let Ok(current_directory) = std::env::current_dir() {
        current_directory.display().to_string()
    } else {
        String::new()
    }
}

pub fn join_paths(current_directory: &str, project_name: &str) -> String {
    let mut path = current_directory.to_string();

    if !path.ends_with("/") {
        path.push('/');
    }

    path.push_str(project_name);

    if !path.ends_with("/") {
        path.push('/');
    }

    path
}
