use std::io::{BufRead, Error};
use regex::Regex;
use crate::config::Config;
use crate::input_source::InputSource;
use crate::output_target::OutputTarget;

pub fn process_input(config: Config, input_sources: Vec<Box<dyn InputSource>>, output: &mut dyn OutputTarget) -> Result<u128, Error> {
    let mut total_match_count = 0;

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

pub fn process_lines<R: BufRead>(filename: &str, reader: R, output: &mut dyn OutputTarget, config: &Config, pattern: &Regex) -> Result<u128, Error> {
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

// Tests for the processing module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_source::TestInputSource;
    use crate::output_target::TestOutputTarget;

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