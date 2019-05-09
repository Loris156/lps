use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    filename: Option<String>,
    root: PathBuf,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, Box<dyn Error>> {
        let filename = match matches.value_of("filename") {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        let root = match matches.value_of("root") {
            Some(s) => PathBuf::from(s),
            None => env::current_dir().unwrap(), // TODO
        };

        Ok(Config { filename, root })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
