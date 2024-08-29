use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    // Define the command line arguments
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
                .num_args(1..)
                .index(2),
        )
        .get_matches();

    // Get the pattern to search for
    let pattern = matches.get_one::<String>("pattern").unwrap();

    // Get the list of files
    if let Some(files) = matches.get_many::<String>("files") {
        // Iterate over each file
        for filename in files {
            if let Ok(file) = File::open(filename) {
                let reader = BufReader::new(file);
                process_lines(reader, pattern);
            } else {
                eprintln!("Error: Could not open file {}", filename);
            }
        }
    } else {
        // No files provided, read from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, pattern);
    }
}

// Function to process lines from a reader
fn process_lines<R: BufRead>(reader: R, pattern: &str) {
    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.contains(pattern) {
                    println!("{}", content);
                }
            }
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}
