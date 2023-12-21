use std::fs;
use walkdir::WalkDir;

pub fn create_project_directory(project_name: &str) {
    // Create the project directory
    fs::create_dir_all(project_name).expect("Failed to create project directory");
}

pub fn copy_template_files(template_folder: &str, project_name: &str, use_typescript: bool) {
    // Determine the correct template folder based on the flag
    let template_folder = if use_typescript {
        template_folder
    } else {
        "template/javascript"
    };

    // Recursively copy the entire directory structure
    for entry in WalkDir::new(template_folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(template_folder).unwrap();
        let dest_path = format!("{}/{}", project_name, relative_path.to_string_lossy());

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path).expect("Failed to create directories");
        } else {
            fs::copy(&src_path, &dest_path).expect("Failed to copy template file");
        }
    }
}
