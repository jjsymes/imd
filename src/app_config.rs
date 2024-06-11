use std::path::PathBuf;
use clap::{arg, command, value_parser};

pub struct AppConfig {
    pub path: PathBuf,
    pub debug: bool,
    pub write: bool,
}

impl AppConfig {
    fn new(path: PathBuf, debug: bool, write: bool) -> AppConfig {
        AppConfig {
            path,
            debug,
            write,
        }
    }

    pub fn from_command_args() -> AppConfig {
        let matches = command!()
            .arg(
                arg!(
                    [path] "Path of the music file"
                )
                .required(true)
                .value_parser(value_parser!(PathBuf))
            )
            .arg(arg!(
                -d --debug ... "Turn debugging information on"
            ))
            .arg(arg!(
                -w --write ... "Apply the matched metadata tags to the file"
            ))
            .get_matches();

        return AppConfig::new(
            matches.get_one::<PathBuf>("path").unwrap().clone(),
            match matches.get_one::<u8>("debug") {
                Some(0) => false,
                _ => true,
            },
            match matches.get_one::<u8>("write") {
                Some(0) => false,
                _ => true,
            },
        );
    }
}
