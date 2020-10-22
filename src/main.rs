// std

// external
use clap::{App, Arg, SubCommand};

// local

fn main() {
    let app = App::new("sstr")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("exec").about("Execute a task defined in the active workspace."),
        )
        .get_matches();

    if let Some(_v) = app.subcommand_matches("exec") {
        println!("Called exec, nothing implemented yet.");
    }
}
