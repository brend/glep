use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use regex::Regex;

// Output mode for the program
enum OutputMode {
    Simple,
    WithFilename,
    SimpleCount,
    WithFilenameCount,
    FilenameOnly,
}

// Command line options
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    #[arg(short)]
    count_only: bool,

    #[arg(short)]
    insensitive: bool,

    #[arg(short='l')]
    filename_only: bool,

    #[arg(short='v')]
    invert_match: bool,

    #[arg()]
    pattern: Regex,

    #[arg()]
    files: Vec<String>,
}

impl Config {
    fn output_mode(&self) -> OutputMode {
        if self.filename_only {
            return OutputMode::FilenameOnly;
        }
        if self.count_only {
            if self.files.len() > 1 {
                OutputMode::WithFilenameCount
            } else {
                OutputMode::SimpleCount
            }
        } else {
            if self.files.len() > 1 {
                OutputMode::WithFilename
            } else {
                OutputMode::Simple
            }
        }
    }
}

fn main() {
    let config = Config::parse();
    
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
        match_count = process_lines(reader, &config, None)?;
    } else {
        // Iterate over each file
        for filename in &config.files {
            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);
                match_count += process_lines(reader, &config, Some(&filename))?;
            } else {
                eprintln!("Error: Could not open file {}", filename);
            }
        }
    }

    Ok(match_count)
}

// Function to process lines from a reader
fn process_lines<R: BufRead>(reader: R, config: &Config, filename: Option<&str>) -> Result<u128, Error> {
    let mut match_count: u128 = 0;
    let output_mode = config.output_mode();
    let pattern = if config.insensitive {
        Regex::new(&format!("(?i){}", config.pattern)).unwrap()
    } else {
        config.pattern.clone()
    };
    for line in reader.lines() {
        match line {
            Ok(content) => {
                let content = if config.insensitive {
                    content.to_lowercase()
                } else {
                    content
                };
                if pattern.is_match(&content) != config.invert_match {
                    match_count += 1;
                    match output_mode {
                        OutputMode::Simple => println!("{}", content),
                        OutputMode::WithFilename => {
                            if let Some(filename) = filename {
                                println!("{}: {}", filename, content);
                            }
                        },
                        OutputMode::FilenameOnly => {
                            if let Some(filename) = filename {
                                println!("{}", filename);
                            } else {
                                println!("(standard input)");
                            }
                            return Ok(1);
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