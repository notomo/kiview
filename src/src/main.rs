#![feature(custom_attribute)]
#![feature(slice_patterns)]

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate serde_json;

mod command;
use command::{ChildCommand, CommandName, CreateCommand, NamedCommand, ParentCommand};

mod repository;

fn main() {
    let app = App::new("kiview")
        .version("0.0.1")
        .subcommand(
            SubCommand::with_name("do")
                .arg(
                    Arg::with_name("arg")
                        .long("arg")
                        .takes_value(true)
                        .default_value("")
                        .required(false),
                )
                .arg(
                    Arg::with_name("cwd")
                        .long("cwd")
                        .takes_value(true)
                        .default_value(".")
                        .required(false),
                )
                .arg(
                    Arg::with_name("targets")
                        .long("targets")
                        .takes_value(true)
                        .multiple(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("complete").arg(
                Arg::with_name("arg")
                    .long("arg")
                    .takes_value(true)
                    .default_value("")
                    .required(false),
            ),
        );

    let matches = app.get_matches();
    match matches.subcommand() {
        ("do", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();
            let command_name = CommandName::from(arg);

            let cwd = cmd.value_of("cwd").unwrap();
            let targets: Vec<&str> = cmd.values_of("targets").unwrap_or_default().collect();

            let path_repository = repository::FilePathRepository {};

            let actions = match command_name {
                CommandName::Quit => NamedCommand { name: command_name }.actions(),
                CommandName::Parent => ParentCommand {
                    current: cwd,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Child => ChildCommand {
                    current: cwd,
                    targets: targets,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Create => CreateCommand {
                    current: cwd,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Unknown => json!([]),
            };

            let output = json!({
                "actions": actions,
            });

            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        ("complete", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();
            println!("{:?}", arg);
        }
        _ => (),
    }
}
