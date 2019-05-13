use std::env;
use std::error::Error;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Config {
    verbose: bool,
    filename: Option<String>,
    content: Option<String>,
    root: PathBuf,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, Box<dyn Error>> {
        let verbose = matches.is_present("verbose");

        let filename = match matches.value_of("filename") {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        let content = match matches.value_of("content") {
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
            content,
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

    traverse(&config, &config.root);

    Ok(())
}

fn traverse(config: &Config, path: &PathBuf) {
    let dir = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            traverse(&config, &path);
            continue;
        }

        let path_str = match path.to_str() {
            Some(p) => p,
            None => {
                continue;
            }
        };

        let file_name = match entry.file_name().to_str() {
            Some(f) => String::from(f),
            None => {
                continue;
            }
        };

        if let Some(search) = &config.filename {
            if !file_name.contains(search) {
                continue;
            }
        }

        if config.content.is_some() {
            content_search(&config, &path);
        } else {
            println!("{}", path_str);
        }
    }
}

fn content_search(config: &Config, path: &PathBuf) {
    assert!(config.content.is_some());

    let path_str = match path.to_str() {
        Some(p) => p,
        None => {
            eprintln!("failed to convert path to UTF-8");
            return;
        }
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut result = Vec::new();

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                break;
            }
        };

        match line.find(config.content.as_ref().unwrap()) {
            Some(pos) => {
                result.push((i + 1, pos, line));
            }
            None => (),
        }
    }

    if !result.is_empty() {
        println!("{}", path_str);
        for (line, column, text) in result {
            println!("  {}:{} {}", line, column, text);
        }
    }
}
