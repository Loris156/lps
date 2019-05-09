use std::env;
use std::error::Error;

use std::io;

use std::fmt;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    verbose: bool,
    filename: Option<String>,
    root: PathBuf,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, Box<dyn Error>> {
        let verbose = matches.is_present("verbose");

        let filename = match matches.value_of("filename") {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        let root = match matches.value_of("root") {
            Some(s) => {
                let path = PathBuf::from(s);
                if !path.is_dir() {
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "working directory is not a directory",
                    )));
                }

                path
            }
            None => env::current_dir()?,
        };

        Ok(Config {
            verbose,
            filename,
            root,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.verbose {
        let root_path = config.root.to_str();
        if root_path.is_none() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid working directory",
            )));
        }

        println!("working directory: {}", root_path.unwrap());
    }

    traverse(&config.root);

    Ok(())
}

fn traverse(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(&path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            traverse(&path)?;
        } else {
            if let Some(name) = path.to_str() {
                println!("{}", name);
            }
        }
    }

    Ok(())
}
