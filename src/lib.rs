use std::env;
use std::error::Error;

use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub struct Config {
    verbose: bool,
    filename: Option<String>,
    content: Option<String>,
    ignore_content_case: bool,
    dop: usize,
    root: PathBuf,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Arc<Config>, Box<dyn Error>> {
        let verbose = matches.is_present("verbose");

        let ignore_filename_case = matches.is_present("ignore-filename-case");

        let filename = match matches.value_of("filename") {
            Some(s) => {
                if ignore_filename_case {
                    Some(String::from(s).to_lowercase())
                } else {
                    Some(String::from(s))
                }
            }
            None => None,
        };

        let ignore_content_case = matches.is_present("ignore-content-case");

        let content = match matches.value_of("content") {
            Some(s) => {
                // If case-insensitive content comparison is requested
                // convert for the whole program lifetime
                if ignore_content_case {
                    Some(String::from(s).to_lowercase())
                } else {
                    Some(String::from(s))
                }
            }
            None => None,
        };

        let dop = match matches.value_of("dop") {
            Some(s) => String::from(s),
            None => num_cpus::get().to_string(),
        };

        let dop = match dop.parse::<usize>() {
            Ok(dop) => dop,
            Err(_) => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid degree of parallelism",
                )));
            }
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

        Ok(Arc::new(Config {
            verbose,
            filename,
            content,
            ignore_content_case,
            dop,
            root,
        }))
    }
}

struct LpsResult {
    file: String,
    lines: Option<Vec<LpsLineResult>>,
}

struct LpsLineResult {
    line: usize,
    column: usize,
    content: String,
}

pub fn run(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    if config.verbose {
        let root_path = config.root.to_str();
        if root_path.is_none() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid working directory",
            )));
        }

        println!("working directory: {}", root_path.unwrap());
        println!("DoP was set to {} threads", config.dop);
    }

    // Get all files that match name, size, attributes, ...
    let files = find_files_by_name(&config, &config.root);

    // Check content in multiple threads
    let (sender, receiver) = mpsc::channel::<LpsResult>();

    content_search(&config, files, sender);

    // Aggregate results
    loop {
        let result = match receiver.recv() {
            Ok(res) => res,
            Err(_) => {
                // This will occur when all threads have finished
                break;
            }
        };

        if result.lines.is_none() {
            // lines is none if no content search was performed, just print the file names
            println!("{}", result.file);
        } else {
            let lines = result.lines.unwrap();
            if !lines.is_empty() {
                println!("{}", result.file);
                for line in lines {
                    println!("  {}:{} {}", line.line, line.column, line.content);
                }
            }
        }
    }

    Ok(())
}

fn find_files_by_name(config: &Config, path: &PathBuf) -> Vec<PathBuf> {
    let mut result = Vec::new();

    let dir = match fs::read_dir(&path) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("{}", err);
            return result;
        }
    };

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            result.append(&mut find_files_by_name(&config, &path));
            continue;
        }

        if let Some(search) = &config.filename {
            let file_name = path.to_string_lossy();
            if file_name.contains(search) {
                result.push(path);
            }
        } else {
            result.push(path);
        }
    }

    result
}

fn content_search(config: &Arc<Config>, files: Vec<PathBuf>, sender: mpsc::Sender<LpsResult>) {
    if config.content.is_none() {
        // Just yield found files if content search is not requested
        for file in files {
            let file = file.to_string_lossy().to_string();
            if sender
                .send(LpsResult {
                    file: file,
                    lines: None,
                })
                .is_err()
            {
                break;
            }
        }
    } else {
        assert!(config.content.is_some());
        for chunk in files.chunks(files.len() / config.dop) {
            let config = config.clone();
            let sender = sender.clone();
            let chunk = chunk.to_vec();

            thread::spawn(move || {
                for file in chunk {
                    let file_path = file.to_string_lossy().to_string();
                    let file = match File::open(file) {
                        Ok(f) => f,
                        Err(_) => {
                            continue;
                        }
                    };

                    let mut found_lines = Vec::new();
                    for (i, line) in BufReader::new(file).lines().enumerate() {
                        let line = match line {
                            Ok(l) => {
                                if config.ignore_content_case {
                                    l.to_lowercase()
                                } else {
                                    l
                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        };

                        if let Some(pos) = line.find(config.content.as_ref().unwrap()) {
                            found_lines.push(LpsLineResult {
                                line: i + 1,
                                column: pos,
                                content: line,
                            });
                        }
                    }

                    if sender
                        .send(LpsResult {
                            file: file_path,
                            lines: Some(found_lines),
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            });
        }
    }
}
