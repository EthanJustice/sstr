// std
use std::env::current_dir;
use std::fs::read_to_string;
use std::process::{exit, Command, Stdio};

// external
use clap::{App, Arg, SubCommand};
use serde_json::{from_str, Value};

// local

fn main() {
    let app = App::new("sstr")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("exec")
                .author(env!("CARGO_PKG_AUTHORS"))
                .about("Execute a task defined in the active workspace.")
                .arg(
                    Arg::with_name("task")
                        .short("t")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(subcommand) = app.subcommand_matches("exec") {
        let current_dir = current_dir().expect("Failed to get the working directory, aborting...");
        let configuration_file = read_to_string(current_dir.join("sstr.json"))
            .expect("Failed to read the configuration file, aborting...");

        let serialised: Value = from_str(configuration_file.as_str())
            .expect("Configuration file is invalid, aborting...");

        let task = subcommand
            .value_of("task")
            .expect("Failed to provide a task, aborting...");
        if serialised.is_object() == true {
            serialised[task]
                .as_array()
                .expect("Configuration file is invalid, aborting...")
                .iter()
                .for_each(|cmd| {
                    let cmd_str = cmd.as_str().unwrap();
                    let script_vec: Vec<String> = cmd_str
                        .split(" ")
                        .take(2)
                        .filter(|i| i.contains("-") == false)
                        .map(|i| String::from(i))
                        .collect();

                    let mut script: String = String::from(script_vec[0].clone());
                    for i in script_vec.iter().skip(1) {
                        let new = format!(" {}", i);
                        script.push_str(new.as_str());
                    }

                    let args: Vec<String> = cmd_str
                        .split("--")
                        .skip(1)
                        .map(|i| {
                            println!("i: {}", i);
                            let new = format!("--{}", i);
                            String::from(new.as_str())
                        })
                        .collect();

                    println!("Running script {:#?} with args {:#?}", script, args);
                    Command::new(script)
                        .args(args)
                        .stdout(Stdio::piped())
                        .output()
                        .expect("Failed to run task.");
                });
        } else {
            println!(
                "Configuration file is invalid (top-level item is not an object), aborting..."
            );
            exit(1)
        }
    }
}
