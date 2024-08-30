use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use regex::Regex;

// Command line options
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// Write only a count of selected lines to standard output
    #[arg(short)]
    count_only: bool,

    /// Perform pattern matching in searches without regard to case
    #[arg(short)]
    insensitive: bool,

    /// Write only the names of files containing selected lines to standard output
    #[arg(short='l')]
    filename_only: bool,

    /// Select lines not matching any of the specified patterns
    #[arg(short='v')]
    invert_match: bool,

    /// Precede each output line by its relative line number in the file, each file starting at line 1
    #[arg(short='n')]
    line_number: bool,

    /// Quiet; do not write anything to standard output
    #[arg(short)]
    quiet: bool,

    /// The string or regular expression to search for
    #[arg()]
    pattern: Regex,

    /// The files to search. If no files are provided, read from stdin
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
    // Create a regex pattern that is case insensitive if the insensitive flag is set
    let pattern = if config.insensitive {
        Regex::new(&format!("(?i){}", config.pattern)).unwrap()
    } else {
        config.pattern.clone()
    };

    if config.files.is_empty() {
        // No files provided, read from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        match_count = process_lines(reader, &config, &pattern, None)?;
        if config.count_only && match_count > 0 && !config.filename_only && !config.quiet {
            println!("{}", match_count);
        }
    } else {
        // Iterate over each file
        for filename in &config.files {
            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);
                match_count += process_lines(reader, &config, &pattern, Some(&filename))?;
                if match_count > 0 && config.count_only && !config.filename_only && !config.quiet {
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
fn process_lines<R: BufRead>(reader: R, config: &Config, pattern: &Regex, filename: Option<&str>) -> Result<u128, Error> {
    let mut line_number: u128 = 0;
    let mut match_count: u128 = 0;
    
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
                        if !config.quiet {
                            if let Some(filename) = filename {
                                println!("{}", filename);
                            } else {
                                println!("(standard input)");
                            }
                        }
                        return Ok(match_count);
                    }

                    // Print the file name unless any of these flags are set: -c, -q
                    if !config.count_only && !config.quiet {
                        if let Some(filename) = filename {
                            if config.files.len() > 1 {
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