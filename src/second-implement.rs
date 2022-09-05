#![allow(unused)]

use clap::Parser;
use command_line_application as clapp;
use std::io::{Error, ErrorKind};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    
    clapp::find_matches(&content, &args.pattern, &std::io::stdout());

    Ok(())
}

// The test function of it
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    clapp::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
