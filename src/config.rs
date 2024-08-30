use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short)]
    pub count_only: bool,
    
    #[arg(short)]
    pub insensitive: bool,
    
    #[arg(short = 'l')]
    pub filename_only: bool,
    
    #[arg(short = 'v')]
    pub invert_match: bool,
    
    #[arg(short = 'n')]
    pub line_number: bool,
    
    #[arg(short)]
    pub quiet: bool,
    
    #[arg()]
    pub pattern: Regex,
    
    #[arg()]
    pub files: Vec<String>,
}
