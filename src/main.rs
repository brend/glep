use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use regex::Regex;

trait InputSource {
    fn reader(&self) -> Box<dyn BufRead>;
    fn filename(&self) -> &str;
}

struct FileInputSource {
    filename: String,
}

impl InputSource for FileInputSource {
    fn reader(&self) -> Box<dyn BufRead> {
        if self.filename == "-" {
            Box::new(io::stdin().lock())
        } else {
            Box::new(BufReader::new(File::open(&self.filename).unwrap()))
        }
    }

    fn filename(&self) -> &str {
        &self.filename
    }
}

#[allow(dead_code)]
struct TestInputSource {
    lines: Vec<String>,
}

impl InputSource for TestInputSource {
    fn reader(&self) -> Box<dyn BufRead> {
        Box::new(io::Cursor::new(self.lines.join("\n")))
    }

    fn filename(&self) -> &str {
        "test"
    }
}

trait OutputTarget {
    fn write(&mut self, message: &str);
}

struct StdoutTarget;

impl OutputTarget for StdoutTarget {
    fn write(&mut self, message: &str) {
        println!("{}", message);
    }
}

#[allow(dead_code)]
struct TestOutputTarget {
    messages: Vec<String>,
}

impl OutputTarget for TestOutputTarget {
    fn write(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }
}

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

    // Create input sources based on the config
    let input_sources: Vec<Box<dyn InputSource>> = if config.files.is_empty() {
        vec![Box::new(FileInputSource {
            filename: "-".to_string(),
        })]
    } else {
        config.files.iter().map(|filename| {
            Box::new(FileInputSource {
                filename: filename.clone(),
            }) as Box<dyn InputSource>
        }).collect()
    };

    let mut output = StdoutTarget;
    
    match process_input(config, input_sources, &mut output) {
        Ok(match_count) => {
            std::process::exit(if match_count > 0 { 0 } else { 1 });
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(2);
        }
    }
}

fn process_input(config: Config, input_sources: Vec<Box<dyn InputSource>>, output: &mut dyn OutputTarget) -> Result<u128, Error> {
    let mut total_match_count = 0;
    
    // Create a regex pattern that is case insensitive if the insensitive flag is set
    let pattern = if config.insensitive {
        Regex::new(&format!("(?i){}", config.pattern)).unwrap()
    } else {
        config.pattern.clone()
    };

    let file_count = input_sources.len();

    for input_source in input_sources {
        let reader = input_source.reader();
        let match_count = process_lines(input_source.filename(), reader, output, &config, &pattern)?;

        if config.count_only && match_count > 0 && !config.quiet {
            if file_count > 1 {
                output.write(&format!("{}:{}", input_source.filename(), match_count));
            } else {
                output.write(&format!("{}", match_count));
            }
        }

        total_match_count += match_count;

        if match_count > 0 && config.filename_only && !config.quiet {
            output.write(&input_source.filename());
            if config.quiet {
                break;
            }
        }
    }

    Ok(total_match_count)
}

fn process_lines<R: BufRead>(filename: &str, reader: R, output: &mut dyn OutputTarget, config: &Config, pattern: &Regex) -> Result<u128, Error> {
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
                        output.write(&format!("{}:{}:{}", filename, line_number, content));
                    } else {
                        output.write(&format!("{}:{}", filename, content));
                    }
                } else {
                    if config.line_number {
                        output.write(&format!("{}:{}", line_number, content));
                    } else {
                        output.write(&format!("{}", content));
                    }
                }
            }
        }
    }

    Ok(match_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_match() {
        let config = Config {
            count_only: false,
            insensitive: false,
            filename_only: false,
            invert_match: false,
            line_number: false,
            quiet: false,
            pattern: Regex::new("foo").unwrap(),
            files: vec![],
        };

        let input_sources: Vec<Box<dyn InputSource>> = vec![Box::new(TestInputSource {
            lines: vec!["bar".to_string()],
        })];
        let mut output = TestOutputTarget { messages: vec![] };

        let result = process_input(config, input_sources, &mut output).unwrap();
        assert_eq!(result, 0);
        assert!(output.messages.is_empty());
    }

    #[test]
    fn test_match() {
        let config = Config {
            count_only: false,
            insensitive: false,
            filename_only: false,
            invert_match: false,
            line_number: false,
            quiet: false,
            pattern: Regex::new("foo").unwrap(),
            files: vec![],
        };

        let input_sources: Vec<Box<dyn InputSource>> = vec![Box::new(TestInputSource {
            lines: vec!["This line contains the word foo!".to_string()],
        })];
        let mut output = TestOutputTarget { messages: vec![] };

        let result = process_input(config, input_sources, &mut output).unwrap();

        assert_eq!(result, 1);
        assert_eq!(output.messages.len(), 1);
        assert_eq!(output.messages[0], "This line contains the word foo!");
    }

    #[test]
    fn test_match_count() {
        let config = Config {
            count_only: true,
            insensitive: false,
            filename_only: false,
            invert_match: false,
            line_number: false,
            quiet: false,
            pattern: Regex::new("foo").unwrap(),
            files: vec![],
        };

        let input_sources: Vec<Box<dyn InputSource>> = vec![Box::new(TestInputSource {
            lines: vec!["This line contains the word foo!".to_string()],
        })];
        let mut output = TestOutputTarget { messages: vec![] };

        let result = process_input(config, input_sources, &mut output).unwrap();

        assert_eq!(result, 1);
        assert_eq!(output.messages.len(), 1);
        assert_eq!(output.messages[0], "1");
    }

    #[test]
    fn test_match_filename_only() {
        let config = Config {
            count_only: false,
            insensitive: false,
            filename_only: true,
            invert_match: false,
            line_number: false,
            quiet: false,
            pattern: Regex::new("foo").unwrap(),
            files: vec!["test".to_string()],
        };

        let input_sources: Vec<Box<dyn InputSource>> = vec![Box::new(TestInputSource {
            lines: vec!["This line contains the word foo!".to_string()],
        })];
        let mut output = TestOutputTarget { messages: vec![] };

        let result = process_input(config, input_sources, &mut output).unwrap();

        assert_eq!(result, 1);
        assert_eq!(output.messages.len(), 1);
        assert_eq!(output.messages[0], "test");
    }

    #[test]
    fn test_match_quiet() {
        let config = Config {
            count_only: false,
            insensitive: false,
            filename_only: false,
            invert_match: false,
            line_number: false,
            quiet: true,
            pattern: Regex::new("foo").unwrap(),
            files: vec![],
        };

        let input_sources: Vec<Box<dyn InputSource>> = vec![Box::new(TestInputSource {
            lines: vec!["This line contains the word foo!".to_string()],
        })];
        let mut output = TestOutputTarget { messages: vec![] };

        let result = process_input(config, input_sources, &mut output).unwrap();

        assert_eq!(result, 1);
        assert_eq!(output.messages.len(), 0);
    }
}