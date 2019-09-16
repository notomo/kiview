use clap::{App, Arg, SubCommand};

use std::fs;

fn main() {
    let app = App::new("kiview")
        .version("0.0.1")
        .subcommand(
            SubCommand::with_name("run").arg(
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
        ("run", Some(cmd)) => {
            let _arg = cmd.value_of("arg").unwrap();

            let directories: Vec<_> = fs::read_dir(".")
                .unwrap()
                .filter(|path| path.as_ref().unwrap().metadata().unwrap().is_dir())
                .map(|path| path.unwrap().file_name())
                .collect();
            for path in directories {
                println!("{}/", path.to_str().unwrap())
            }

            let files: Vec<_> = fs::read_dir(".")
                .unwrap()
                .filter(|path| !path.as_ref().unwrap().metadata().unwrap().is_dir())
                .map(|path| path.unwrap().file_name())
                .collect();
            for path in files {
                println!("{}", path.to_str().unwrap())
            }
        }
        ("complete", Some(cmd)) => {
            let arg = cmd.value_of("arg").unwrap();
            println!("{:?}", arg);
        }
        _ => (),
    }
}
