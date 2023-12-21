use clap::{App, Arg};

pub fn create_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("create")
        .about("Create a new Express.js project")
        .arg(Arg::with_name("project_name").required(true))
        .arg(
            Arg::with_name("typescript")
                .long("typescript")
                .help("Use TypeScript"),
        )
}
