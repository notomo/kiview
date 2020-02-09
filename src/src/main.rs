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
use command::{command_complete, parse_command_actions, Current};

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
            SubCommand::with_name("complete")
                .arg(
                    Arg::with_name("arg")
                        .long("arg")
                        .takes_value(true)
                        .default_value("")
                        .required(false),
                )
                .arg(
                    Arg::with_name("line")
                        .long("line")
                        .takes_value(true)
                        .default_value("")
                        .required(false),
                ),
        );

    let matches = app.get_matches();
    match matches.subcommand() {
        ("do", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();

            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line).unwrap();
            let current: Current = serde_json::from_str(&line).unwrap();

            match parse_command_actions(arg, current) {
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
            let line = cmd.value_of("line").unwrap();
            let output = command_complete(&arg, &line).join("\n");
            println!("{}", output);
        }
        _ => (),
    }
}
