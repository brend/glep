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
    #[arg(short = 'l')]
    filename_only: bool,

    /// Select lines not matching any of the specified patterns
    #[arg(short = 'v')]
    invert_match: bool,

    /// Precede each output line by its relative line number in the file, each file starting at line 1
    #[arg(short = 'n')]
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
    let mut total_match_count = 0;
    
    // Create a regex pattern that is case insensitive if the insensitive flag is set
    let pattern = if config.insensitive {
        Regex::new(&format!("(?i){}", config.pattern)).unwrap()
    } else {
        config.pattern.clone()
    };

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let file_count = files.len();

    for filename in files {
        let reader: Box<dyn BufRead> = if filename == "-" {
            Box::new(io::stdin().lock())
        } else {
            Box::new(BufReader::new(File::open(&filename)?))
        };

        let match_count = process_lines(reader, &config, &pattern, &filename)?;

        if config.count_only && match_count > 0 && !config.quiet {
            if file_count > 1 {
                println!("{}:{}", filename, match_count);
            } else {
                println!("{}", match_count);
            }
        }

        total_match_count += match_count;

        if match_count > 0 && config.filename_only && !config.quiet {
            println!("{}", filename);
            if config.quiet {
                break;
            }
        }
    }

    Ok(total_match_count)
}

fn process_lines<R: BufRead>(reader: R, config: &Config, pattern: &Regex, filename: &str) -> Result<u128, Error> {
    let mut line_number: u128 = 0;
    let mut match_count: u128 = 0;
    
    for line in reader.lines() {
        line_number += 1;
        let content = line?;

        let matched = pattern.is_match(&content) != config.invert_match;
        if matched {
            match_count += 1;
            if config.quiet {
                return Ok(match_count);
            }
            if config.filename_only {
                return Ok(match_count);
            }
            if !config.count_only {
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
            }
        }
    }

    Ok(match_count)
}
