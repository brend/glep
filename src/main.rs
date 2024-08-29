use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use regex::Regex;

// Output mode for the program
enum OutputMode {
    Simple,
    WithFilename,
    SimpleCount,
    WithFilenameCount,
}

// Command line options
struct Config {
    pattern: Regex,
    files: Vec<String>,
    output_mode: OutputMode,
}

impl Config {
    // Function to parse command line arguments and return a Config instance
    fn from_args() -> Self {
        let matches = Command::new("glep")
            .version("1.0")
            .author("Philipp Brendel <waldrumpus@gmail.com>")
            .about("Echo lines containing a specific pattern")
            .arg(
                Arg::new("pattern")
                    .help("The pattern to search for")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("files")
                    .help("Files to search in")
                    .num_args(0..) // Indicates zero or more arguments can be provided
                    .index(2),
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("count")
                    .action(clap::ArgAction::SetTrue)
                    .help("Write only a count of selected lines to standard output"),
            )
            .get_matches();

        // Extract the pattern and files from the matches
        let pattern_string = matches.get_one::<String>("pattern").unwrap().clone();
        let pattern = Regex::new(&pattern_string).unwrap();
        let files = matches
            .get_many::<String>("files")
            .map(|vals| vals.cloned().collect())
            .unwrap_or_else(Vec::new);
        let count = matches.get_flag("count");
        let output_mode = if files.len() > 1 {
            if count {
                OutputMode::WithFilenameCount
            } else {
                OutputMode::WithFilename
            }
        } else {
            if count {
                OutputMode::SimpleCount
            } else {
                OutputMode::Simple
            }
        };

        Config { pattern, files, output_mode }
    }
}

fn main() {
    let config = Config::from_args();
    
    match process_input(config) {
        Ok(match_count) => {
            std::process::exit(if match_count > 0 { 0 } else { 1 });
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(2);
        }
    }
}

fn process_input(config: Config) -> Result<u128, Error> {
    let mut match_count = 0;
    // If file arguments have not been provided, read from stdin
    if config.files.is_empty() {
        // No files provided, read from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        match_count = process_lines(reader, &config.pattern, &config.output_mode, None)?;
    } else {
        // Iterate over each file
        for filename in config.files {
            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);
                match_count += process_lines(reader, &config.pattern, &config.output_mode, Some(&filename))?;
            } else {
                eprintln!("Error: Could not open file {}", filename);
            }
        }
    }

    Ok(match_count)
}

// Function to process lines from a reader
fn process_lines<R: BufRead>(reader: R, pattern: &Regex, output_mode: &OutputMode, filename: Option<&str>) -> Result<u128, Error> {
    let mut match_count: u128 = 0;
    for line in reader.lines() {
        match line {
            Ok(content) => {
                if pattern.is_match(&content) {
                    match_count += 1;
                    match output_mode {
                        OutputMode::Simple => println!("{}", content),
                        OutputMode::WithFilename => {
                            if let Some(filename) = filename {
                                println!("{}: {}", filename, content);
                            }
                        },
                        _ => (),
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    match output_mode {
        OutputMode::SimpleCount => println!("{}", match_count),
        OutputMode::WithFilenameCount => {
            if let Some(filename) = filename {
                println!("{}: {}", filename, match_count);
            }
        },
        _ => (),
    }

    Ok(match_count)
}