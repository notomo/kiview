#![feature(custom_attribute)]
#![feature(slice_patterns)]
use clap::{App, Arg, SubCommand};
use std::path::Path;

#[macro_use]
extern crate serde_json;

use std::fs;

mod command;
use command::{CommandName, NamedCommand};

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
            let targets: Vec<_> = cmd.values_of("targets").unwrap_or_default().collect();

            let actions = match command_name {
                CommandName::Quit => NamedCommand { name: command_name }.actions(),
                CommandName::Parent => {
                    let path = Path::new(cwd).parent().unwrap_or_else(|| Path::new(cwd));
                    let paths = get_paths(path);
                    json!([{
                          "name": "update",
                          "args": paths,
                          "options": {
                              "cwd": path.canonicalize().unwrap(),
                          },
                    }])
                }
                CommandName::Child => {
                    let path = Path::new(cwd);
                    let dirs: Vec<_> = targets
                        .iter()
                        .map(|target| Path::new(cwd).join(target))
                        .filter(|path| {
                            path.metadata()
                                .and_then(|metadata| Ok(metadata.is_dir()))
                                .unwrap_or(false)
                        })
                        .collect();

                    match &dirs[..] {
                        [] => {
                            let files: Vec<_> = targets
                                .iter()
                                .map(|target| Path::new(cwd).join(target))
                                .filter(|path| {
                                    path.metadata()
                                        .and_then(|metadata| Ok(!metadata.is_dir()))
                                        .unwrap_or(false)
                                })
                                .collect();

                            json!([{
                              "name": "open",
                              "args": files,
                              "options": {
                                  "cwd": path.canonicalize().unwrap(),
                              },
                            }])
                        }
                        _ => {
                            let path = dirs[0].as_path();
                            let paths = get_paths(path);
                            json!([{
                              "name": "update",
                              "args": paths,
                              "options": {
                                  "cwd": path.canonicalize().unwrap(),
                              }
                            }])
                        }
                    }
                }
                CommandName::Create => {
                    let path = Path::new(cwd);
                    let paths = get_paths(path);
                    json!([{
                          "name": "create",
                          "args": paths,
                          "options": {
                              "cwd": path.canonicalize().unwrap(),
                          },
                    }])
                }
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

fn get_paths(dir_path: &Path) -> Vec<String> {
    let directories: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .filter(|path| path.as_ref().unwrap().metadata().unwrap().is_dir())
        .map(|path| format!("{}/", path.unwrap().file_name().to_str().unwrap()))
        .collect();

    let files: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
        .map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
        .collect();

    [&directories[..], &files[..]].concat()
}
