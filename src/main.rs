use clap::{App, Arg};
use lps::Config;

fn main() {
    let matches = App::new("lps")
        .version("1.0.1")
        .author("Loris Leitner (Loris156)")
        .about("High-speed parallelized searching")
        .arg(
            Arg::with_name("filename")
                .short("n")
                .long("name")
                .takes_value(true)
                .value_name("FILENAME")
                .help("Filename pattern"),
        )
        .arg(
            Arg::with_name("ignore-filename-case")
                .short("b")
                .long("ignore-filename-case")
                .requires("filename")
                .help("Ignores casing of filename"),
        )
        .arg(
            Arg::with_name("content")
                .short("c")
                .long("content")
                .takes_value(true)
                .value_name("TEXT")
                .help("File content"),
        )
        .arg(
            Arg::with_name("ignore-content-case")
                .short("x")
                .long("ignore-content-case")
                .requires("content")
                .help("Ignores casing of content"),
        )
        .arg(
            Arg::with_name("dop")
                .short("d")
                .long("dop")
                .requires("content")
                .takes_value(true)
                .value_name("THREAD COUNT")
                .help("Degree of parallelism (defaults to logical core count)"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose output"),
        )
        .arg(
            Arg::with_name("root")
                .help("Root search directory")
                .value_name("ROOT"),
        )
        .get_matches();

    let config = Config::new(&matches).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    lps::run(config).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });
}
