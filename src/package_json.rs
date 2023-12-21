use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_package_json(project_name: &str) {
    let mut dependencies = serde_json::Map::new();

    // Add the required dependencies to the map
    dependencies.insert(
        "express".to_string(),
        serde_json::Value::String("latest".to_string()),
    );
    dependencies.insert(
        "dotenv".to_string(),
        serde_json::Value::String("latest".to_string()),
    );

    let package_json = PackageJson {
        name: project_name.to_string(),
        version: "1.0.0".to_string(),
        description: "Express.js project generated with Rust CLI".to_string(),
        main: "app.js".to_string(),
        dependencies,
    };

    let package_json_content =
        serde_json::to_string_pretty(&package_json).expect("Failed to serialize package.json");

    let package_json_path = format!("{}/package.json", project_name);
    let mut file = File::create(&package_json_path).expect("Failed to create package.json");
    file.write_all(package_json_content.as_bytes())
        .expect("Failed to write package.json");
}

// Add the PackageJson struct for serde serialization
#[derive(Serialize, Deserialize)]
struct PackageJson {
    name: String,
    version: String,
    description: String,
    main: String,
    dependencies: serde_json::Map<String, serde_json::Value>,
}
