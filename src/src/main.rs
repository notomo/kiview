#![feature(custom_attribute)]
#![feature(slice_patterns)]

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate serde_json;

mod command;
use command::{CommandName, CommandOptions};

mod repository;

fn main() {
    let app = App::new("kiview")
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
                    Arg::with_name("current_path")
                        .long("current-path")
                        .takes_value(true)
                        .default_value(".")
                        .required(false),
                )
                .arg(
                    Arg::with_name("line_number")
                        .long("line-number")
                        .takes_value(true)
                        .default_value("1")
                        .required(false),
                )
                .arg(
                    Arg::with_name("current_target")
                        .long("current-target")
                        .takes_value(true)
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
            let command_opts = CommandOptions::new(arg);

            let current_path = cmd.value_of("current_path").unwrap();
            let line_number = cmd.value_of("line_number").unwrap().parse().unwrap();
            let current_target = cmd.value_of("current_target");
            let targets: Vec<&str> = cmd.values_of("targets").unwrap_or_default().collect();

            let path_repository = repository::FilePathRepository {};

            let actions = match command_name {
                CommandName::Quit => command::NamedCommand { name: command_name }.actions(),
                CommandName::Parent => command::ParentCommand {
                    current_path: current_path,
                    line_number: line_number,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Child => command::ChildCommand {
                    current_path: current_path,
                    line_number: line_number,
                    current_target: current_target,
                    targets: targets,
                    opts: &command_opts,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Create => command::CreateCommand {
                    current_path: current_path,
                    line_number: line_number,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Go => command::GoCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::New => command::NewCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    path_repository: &path_repository,
                }
                .actions(),
                CommandName::Remove => command::RemoveCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    targets: targets,
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
