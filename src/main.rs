mod config;
mod input_source;
mod output_target;
mod processing;

use clap::Parser;
use crate::config::Config;
use crate::input_source::{InputSource, FileInputSource};
use crate::output_target::StdoutTarget;
use crate::processing::process_input;

fn main() {
    let config = Config::parse();

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
