use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use regex::Regex;

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

    #[arg(short='n')]
    line_number: bool,

    #[arg()]
    pattern: Regex,

    #[arg()]
    files: Vec<String>,
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
        if config.count_only && match_count > 0 && !config.filename_only {
            println!("{}", match_count);
        }
    } else {
        // Iterate over each file
        for filename in &config.files {
            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);
                match_count += process_lines(reader, &config, Some(&filename))?;
                if match_count > 0 && config.count_only && !config.filename_only {
                    if config.files.len() > 1 {
                        println!("{}:{}", filename, match_count);
                    } else {
                        println!("{}", match_count);
                    }
                }
            } else {
                eprintln!("Error: Could not open file {}", filename);
            }
        }
    }

    Ok(match_count)
}

// Function to process lines from a reader
fn process_lines<R: BufRead>(reader: R, config: &Config, filename: Option<&str>) -> Result<u128, Error> {
    let mut line_number: u128 = 0;
    let mut match_count: u128 = 0;
    let pattern = if config.insensitive {
        Regex::new(&format!("(?i){}", config.pattern)).unwrap()
    } else {
        config.pattern.clone()
    };
    for line in reader.lines() {
        line_number += 1;
        match line {
            Ok(content) => {
                // Lowercase the content if the insensitive flag is set
                let content = if config.insensitive {
                    content.to_lowercase()
                } else {
                    content
                };

                // Check if the pattern matches the content
                if pattern.is_match(&content) != config.invert_match {
                    match_count += 1;

                    // If the file name only flag is set, print the filename and return
                    if config.filename_only {
                        if let Some(filename) = filename {
                            println!("{}", filename);
                        } else {
                            println!("(standard input)");
                        }
                        return Ok(match_count);
                    }

                    // Print the file name unless the count only flag is set
                    if !config.count_only {
                        if let Some(filename) = filename {
                            if config.line_number {
                                println!("{}:{}:{}", filename, line_number, content);
                            } else {
                                println!("{}:{}", filename, content);
                            }
                        } else {
                            if config.line_number {
                                println!("{}:{}", line_number, content);
                            } else {
                                println!("{}", content);
                            }
                        }
                    }
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    Ok(match_count)
}