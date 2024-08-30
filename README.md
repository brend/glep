# GLEP -- Rust Grep-like Tool

This Rust project is a command-line tool that mimics some of the functionality of `grep`, a powerful search utility found in Unix/Linux systems. The tool allows users to search for patterns in files or standard input and offers various options for customizing the search behavior.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Command-Line Options](#command-line-options)
- [Examples](#examples)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

- Search for patterns in files or standard input.
- Support for regular expressions.
- Case-insensitive search.
- Inverted match (select lines not matching the pattern).
- Option to display line numbers.
- Quiet mode to suppress output.
- Count the number of matching lines.
- Display only filenames that contain matching lines.

## Installation

To build and run this project, you need to have [Rust](https://www.rust-lang.org/) installed on your system.

Clone the repository:

```bash
git clone https://github.com/yourusername/rust-grep-tool.git
cd rust-grep-tool
```

Build the project using Cargo:

```bash
Code kopieren
cargo build --release
```

This will create an executable in the target/release directory.

Usage
To use the tool, run the following command:

```bash
Code kopieren
cargo run -- <options> <pattern> <files...>
```

You can also run the compiled binary directly:

```bash
./target/release/rust-grep-tool <options> <pattern> <files...>
Command-Line Options
-c, --count-only: Write only a count of selected lines to standard output.
-i, --insensitive: Perform case-insensitive pattern matching.
-l, --filename-only: Write only the names of files containing selected lines.
-v, --invert-match: Select lines not matching any of the specified patterns.
-n, --line-number: Precede each output line by its relative line number in the file.
-q, --quiet: Quiet mode; do not write anything to standard output.
```

## Examples
Search for the pattern "foo" in file.txt:

```bash
cargo run -- foo file.txt
```

Search case-insensitively:

```bash
cargo run -- -i foo file.txt
```

Count the number of matching lines:

```bash
cargo run -- -c foo file.txt
```

Search in multiple files:

```bash
cargo run -- foo file1.txt file2.txt
```

Search with inverted match (lines not containing "foo"):

```bash
cargo run -- -v foo file.txt
```

## Testing
This project includes unit tests to ensure correctness. To run the tests, use the following command:

```bash
cargo test
```

The tests are located in the respective module files, and they cover various cases such as matching lines, counting matches, and handling different input configurations.

## Project Structure
Here’s an overview of the project structure:

```plaintext
src/
├── config.rs         // Command-line argument parsing and configuration
├── input_source.rs   // Input source handling (files, stdin)
├── output_target.rs  // Output target handling (stdout, test output)
├── processing.rs     // Core logic for processing input and pattern matching
├── main.rs           // Entry point of the application
└── lib.rs            // Module declarations and re-exports
tests/
├── test_processing.rs // Additional tests (if applicable)
```

## Contributing
Contributions are welcome! If you have suggestions, feature requests, or bug reports, please open an issue or submit a pull request.

Fork the repository.
Create a new branch (git checkout -b feature-branch).
Commit your changes (git commit -am 'Add new feature').
Push to the branch (git push origin feature-branch).
Open a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

```markdown
### Key Points to Customize
- **Repository URL**: Replace `https://github.com/yourusername/rust-grep-tool.git` with your actual GitHub repository URL.
- **Project Name**: Replace "Rust Grep-like Tool" with the actual name of your project.
- **Usage Examples**: Tailor the examples section to showcase typical use cases of your tool.
- **Contributing**: Adjust the contributing section based on your preferences for how contributions should be handled.
- **License**: Ensure that the correct license type and link are included. If you’re using a different license, update the name and file accordingly.

This `README.md` should provide a comprehensive guide to anyone using or contributing to your project
```