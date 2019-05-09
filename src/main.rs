use clap::{App, Arg, SubCommand};
use lps::Config;

fn main() {
    let matches = App::new("lps")
        .version("0.1")
        .author("Loris Leitner (Loris156)")
        .about("High-speed parallelized searching")
        .arg(
            Arg::with_name("filename")
                .short("n")
                .long("name")
                .value_name("FILENAME")
                .help("Filename pattern")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("root")
                .help("Root search directory")
                .value_name("ROOT"),
        )
        .get_matches();

    let config = Config::new(&matches).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    lps::run(config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
