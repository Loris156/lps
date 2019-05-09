pub struct Config {
    filename: Option<String>,
    root: Option<String>,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Config {
        let filename = match matches.value_of("filename") {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        let root = match matches.value_of("root") {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        Config { filename, root }
    }
}

pub fn run(config: Config) {}
