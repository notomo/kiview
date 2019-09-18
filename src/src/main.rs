use clap::{App, Arg, SubCommand};
use std::path::Path;

#[macro_use]
extern crate serde_json;

use std::fs;

fn main() {
    let app = App::new("kiview")
        .version("0.0.1")
        .subcommand(
            SubCommand::with_name("run")
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
                    Arg::with_name("target")
                        .long("target")
                        .takes_value(true)
                        .default_value(".")
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
        ("run", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();
            let cwd = cmd.value_of("cwd").unwrap();
            let target = cmd.value_of("target").unwrap();

            let child = Path::new(cwd).join(target);
            let dir = match arg {
                "parent" => Path::new(cwd).parent().unwrap_or(Path::new(cwd)),
                "child" if child.metadata().unwrap().is_dir() => child.as_path(),
                _ => Path::new(cwd),
            };

            let actions = match arg {
                "child" if !child.metadata().unwrap().is_dir() => Some(json!([{
                    "name": "open",
                    "target": child.canonicalize().unwrap().to_str().unwrap(),
                }])),
                _ => None,
            };

            let directories: Vec<_> = fs::read_dir(dir)
                .unwrap()
                .filter(|path| path.as_ref().unwrap().metadata().unwrap().is_dir())
                .map(|path| format!("{}/", path.unwrap().file_name().to_str().unwrap()))
                .collect();

            let files: Vec<_> = fs::read_dir(dir)
                .unwrap()
                .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
                .map(|path| format!("{}", path.unwrap().file_name().to_str().unwrap()))
                .collect();

            let paths = [&directories[..], &files[..]].concat();

            let need_update = &actions.is_none();

            let output = json!({
                "lines": paths,
                "cwd": dir.canonicalize().unwrap().to_str().unwrap(),
                "actions": actions.unwrap_or_else(|| json!([])),
                "need_update": need_update,
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
