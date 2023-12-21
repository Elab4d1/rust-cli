use dialoguer::Confirm;
use std::process::{Command, Stdio};

pub fn confirm_npm_install() -> bool {
    return Confirm::new()
        .with_prompt("Do you want to run 'npm install' to install dependencies?")
        .default(false)
        .show_default(true)
        .interact()
        .expect("Confirmation failed");
}

pub fn run_npm_install(project_name: &str) {
    // Install dependencies using npm
    let npm_install = Command::new("cmd")
        .arg("/C")
        .arg("npm install")
        .current_dir(&project_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match npm_install {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait for npm install");
            if status.success() {
                println!("Dependencies installed successfully.");
            } else {
                eprintln!("Failed to install dependencies.");
            }
        }
        Err(e) => {
            eprintln!("Error running npm install: {}", e);
        }
    }
}
