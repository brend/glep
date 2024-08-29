use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum OutputMode {
    Simple,
    WithFilename,
}

// Command line options
struct Config {
    pattern: String,
    files: Vec<String>,
    output_mode: OutputMode,
}

impl Config {
    // Function to parse command line arguments and return a Config instance
    fn from_args() -> Self {
        let matches = Command::new("echo_cli")
            .version("1.0")
            .author("Your Name <your.email@example.com>")
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
            .get_matches();

        // Extract the pattern and files from the matches
        let pattern = matches.get_one::<String>("pattern").unwrap().clone();
        let files = matches
            .get_many::<String>("files")
            .map(|vals| vals.cloned().collect())
            .unwrap_or_else(Vec::new);
        let output_mode = if files.len() > 1 {
            OutputMode::WithFilename
        } else {
            OutputMode::Simple
        };

        Config { pattern, files, output_mode }
    }
}

fn main() {
    // Parse the command line arguments
    let config = Config::from_args();

    // If file arguments have not been provided, read from stdin
    if config.files.is_empty() {
        // No files provided, read from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, &config.pattern, &config.output_mode, None);
    } else {
        // Iterate over each file
        for filename in config.files {
            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);
                process_lines(reader, &config.pattern, &config.output_mode, Some(&filename));
            } else {
                eprintln!("Error: Could not open file {}", filename);
            }
        }
    }
}

// Function to process lines from a reader
fn process_lines<R: BufRead>(reader: R, pattern: &str, output_mode: &OutputMode, filename: Option<&str>) {
    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.contains(pattern) {
                    match output_mode {
                        OutputMode::Simple => println!("{}", content),
                        OutputMode::WithFilename => {
                            if let Some(filename) = filename {
                                println!("{}: {}", filename, content);
                            }
                        }
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}
