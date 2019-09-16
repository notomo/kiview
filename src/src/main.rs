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

            let dir = match arg {
                "parent" => Path::new(cwd).parent().unwrap_or(Path::new(cwd)),
                _ => Path::new(cwd),
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

            let output = json!({
                "lines": paths,
                "cwd": dir.to_str().unwrap(),
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
