#![feature(box_syntax)]
#![feature(try_trait)]

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate serde_json;

mod command;
use command::{Command, CommandName, CommandOptions};

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
                )
                .arg(
                    Arg::with_name("registered_targets")
                        .long("registered")
                        .takes_value(true)
                        .multiple(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("has_cut")
                        .long("has-cut")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("created")
                        .long("created")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("opened")
                        .long("opened")
                        .takes_value(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("next_sibling_line_number")
                        .long("next-sibling-line-number")
                        .takes_value(true)
                        .default_value("1")
                        .required(false),
                )
                .arg(
                    Arg::with_name("depth")
                        .long("depth")
                        .takes_value(true)
                        .default_value("0")
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
            let registered_targets: Vec<&str> = cmd
                .values_of("registered_targets")
                .unwrap_or_default()
                .collect();
            let has_cut = cmd.is_present("has_cut");
            let created = cmd.is_present("created");
            let opened = cmd.is_present("opened");
            let next_sibling_line_number = cmd
                .value_of("next_sibling_line_number")
                .unwrap()
                .parse()
                .unwrap();
            let depth = cmd.value_of("depth").unwrap().parse().unwrap();

            let path_repository = repository::FilePathRepository {};
            let dispatcher = repository::Dispatcher {};

            let actions = match &command_name {
                CommandName::Quit => box command::QuitCommand {},
                CommandName::Parent => box command::ParentCommand {
                    current_path: current_path,
                    line_number: line_number,
                    path_repository: &path_repository,
                    dispatcher: dispatcher,
                } as Box<dyn Command>,
                CommandName::Child => box command::ChildCommand {
                    line_number: line_number,
                    current_target: current_target,
                    targets: targets,
                    opts: &command_opts,
                    path_repository: &path_repository,
                    dispatcher: dispatcher,
                } as Box<dyn Command>,
                CommandName::Go => box command::GoCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    path_repository: &path_repository,
                    dispatcher: dispatcher,
                    created: created,
                } as Box<dyn Command>,
                CommandName::New => box command::NewCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    path_repository: &path_repository,
                } as Box<dyn Command>,
                CommandName::Remove => box command::RemoveCommand {
                    current_path: current_path,
                    line_number: line_number,
                    opts: &command_opts,
                    targets: targets,
                    path_repository: &path_repository,
                } as Box<dyn Command>,
                CommandName::Copy => {
                    box command::CopyCommand { targets: targets } as Box<dyn Command>
                }
                CommandName::Cut => {
                    box command::CutCommand { targets: targets } as Box<dyn Command>
                }
                CommandName::Paste => box command::PasteCommand {
                    current_path: current_path,
                    line_number: line_number,
                    path_repository: &path_repository,
                    registered_targets: registered_targets,
                    has_cut: has_cut,
                } as Box<dyn Command>,
                CommandName::Rename => box command::RenameCommand {
                    current_path: current_path,
                    line_number: line_number,
                    current_target: current_target,
                    path_repository: &path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::ToggleTree => box command::ToggleTreeCommand {
                    current_path: current_path,
                    line_number: line_number,
                    current_target: current_target,
                    targets: targets,
                    opts: &command_opts,
                    path_repository: &path_repository,
                    dispatcher: dispatcher,
                    next_sibling_line_number: next_sibling_line_number,
                    opened: opened,
                    depth: depth,
                } as Box<dyn Command>,
                CommandName::Unknown => {
                    box command::UnknownCommand { command_name: &arg } as Box<dyn Command>
                }
            }
            .actions();

            match actions {
                Ok(actions) => {
                    let output = json!({
                        "actions": actions,
                    });
                    println!("{}", serde_json::to_string_pretty(&output).unwrap());
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
        }
        ("complete", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();
            println!("{:?}", arg);
        }
        _ => (),
    }
}
