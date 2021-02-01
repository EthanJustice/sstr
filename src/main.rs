#![feature(command_access)]

// std
use std::env::{current_dir, vars};
use std::fs::read_to_string;
use std::process::{exit, Command, Stdio};
use std::{collections::HashMap, env::var};

// external
use clap::{App, Arg, SubCommand};
use serde_json::{from_str, Value};

// local

fn main() {
    let app = App::new("sstr")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("run")
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

    if let Some(subcommand) = app.subcommand_matches("run") {
        let current_dir = current_dir().expect("Failed to get the working directory, aborting...");

        let configuration_file = read_to_string(current_dir.join("sstr.json"))
            .expect("Failed to read the configuration file, aborting...");

        let serialised: Value = from_str(configuration_file.as_str())
            .expect("Configuration file is invalid, aborting...");

        let task = subcommand
            .value_of("task")
            .expect("Failed to provide a task, aborting...");
        if serialised.is_object() == true {
            let mut all = serialised[task]
                .as_array()
                .expect("Configuration file is invalid, aborting...")
                .clone();
            let cmd = &all[0].to_string();

            all.remove(0);

            let mut path = var("PATH").unwrap_or(
                std::env::current_dir()
                    .expect("Failed to get the current working directory.")
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            path.push_str(cmd);
            println!("{:#?}", path);

            let mut command = Command::new(path);

            command.current_dir(
                std::env::current_dir().expect("Failed to get the current working directory."),
            );

            let vars: HashMap<String, String> = vars().collect();
            command.envs(vars);

            if all.len() > 0 {
                let all: Vec<String> = all.iter().map(|f| f.to_string()).collect();

                for ref arg in all.iter() {
                    command.arg(arg);
                }
            }

            match command
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .output()
            {
                Ok(out) => {
                    println!(
                        "Task {} with args {:#?} successfully exited with code {}",
                        cmd,
                        all,
                        out.status.to_string()
                    );
                }
                Err(err) => {
                    eprintln!("Failed to run task {}: {}", cmd, err.to_string());
                    panic!()
                }
            }
        } else {
            println!(
                "Configuration file is invalid (top-level item is not an object), aborting..."
            );
            exit(1)
        }
    } else {
        eprintln!("Invalid command!");
    }
}
