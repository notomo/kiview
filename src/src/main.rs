#![feature(box_syntax)]
#![feature(try_trait)]

extern crate itertools;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate log4rs;

mod command;
mod handler;
mod repository;
mod state;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use nvim_rs::create::tokio as create;
use std::error::Error;

#[tokio::main]
async fn main() {
    {
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
    }

    let handler = handler::NeovimHandler {};
    let (nvim, io_handler) = create::new_parent(handler).await;
    match io_handler.await {
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {
            if !err.is_reader_error() {
                nvim.err_writeln(&format!("Error: '{}'", err))
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("Well, dang... '{}'", e);
                    });
            }

            if !err.is_channel_closed() {
                eprintln!("Error: '{}'", err);

                let mut source = err.source();
                while let Some(e) = source {
                    eprintln!("Caused by: '{}'", e);
                    source = e.source();
                }
            }
        }
        Ok(Ok(())) => {}
    }
}
