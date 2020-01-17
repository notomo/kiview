#![feature(box_syntax)]
#![feature(try_trait)]

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate serde_json;

use std::io::prelude::*;

extern crate itertools;

#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

mod command;
use command::{Command, CommandName, CommandOptions, Current};

mod repository;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("/tmp/kiview.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )
        .unwrap();
    log4rs::init_config(config).unwrap();

    let app = App::new("kiview")
        .subcommand(
            SubCommand::with_name("do").arg(
                Arg::with_name("arg")
                    .long("arg")
                    .takes_value(true)
                    .default_value("")
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

            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line).unwrap();
            let current: Current = serde_json::from_str(&line).unwrap();

            let dispatcher = repository::Dispatcher {};
            let path_repository = dispatcher.path_repository();

            let actions = match &command_name {
                CommandName::Quit => box command::QuitCommand {},
                CommandName::Parent => box command::ParentCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                } as Box<dyn Command>,
                CommandName::Child => box command::ChildCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::Go => box command::GoCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::New => box command::NewCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::Remove => box command::RemoveCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::Copy => {
                    box command::CopyCommand { current: current } as Box<dyn Command>
                }
                CommandName::Cut => {
                    box command::CutCommand { current: current } as Box<dyn Command>
                }
                CommandName::Paste => box command::PasteCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::Rename => box command::RenameCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::MultipleRename => box command::MultipleRenameCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::ToggleTree => box command::ToggleTreeCommand {
                    current: current,
                    dispatcher: dispatcher,
                    path_repository: path_repository,
                    opts: &command_opts,
                } as Box<dyn Command>,
                CommandName::ToggleSelection => {
                    box command::ToggleSelectionCommand { current: current } as Box<dyn Command>
                }
                CommandName::ToggleAllSelection => {
                    box command::ToggleAllSelectionCommand {} as Box<dyn Command>
                }
                CommandName::Back => {
                    box command::BackCommand { current: current } as Box<dyn Command>
                }
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
                    println!("{}", serde_json::to_string(&output).unwrap());
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
