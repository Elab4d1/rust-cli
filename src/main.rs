extern crate clap;
use clap::App;

mod cli;
mod file_operations;
mod npm_install;
mod package_json;

fn main() {
    let matches = App::new("express-cli")
        .version("1.0")
        .author("Hassan AL Meftah")
        .about("Generate Express.js Server boilerplate")
        .subcommand(cli::create_subcommand())
        .get_matches();

    match matches.subcommand() {
        ("create", Some(create_matches)) => {
            let project_name = create_matches.value_of("project_name").unwrap();
            let use_typescript = create_matches.is_present("typescript");

            // Create the project directory
            file_operations::create_project_directory(project_name);

            // Determine the template folder based on TypeScript option
            let template_folder = if use_typescript {
                "template/typescript"
            } else {
                "template/javascript"
            };

            // Copy the template files to the project directory
            file_operations::copy_template_files(template_folder, project_name, use_typescript);

            // Generate package.json and install dependencies
            package_json::generate_package_json(project_name);

            // Ask for confirmation to run npm install
            if npm_install::confirm_npm_install() {
                npm_install::run_npm_install(project_name);
            }

            println!(
                "Created Express.js project '{}' using {} template.",
                project_name,
                if use_typescript {
                    "TypeScript"
                } else {
                    "JavaScript"
                }
            );
            println!("");
            // Print the "run cd {project_name}" message
            println!(
                "Run 'cd {}' to navigate to your project directory.",
                project_name
            );
        }
        _ => println!("No subcommand specified."),
    }
}
