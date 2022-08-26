#![allow(unused)]

use clap::Parser;
use std::io::{BufReader, BufRead, Read};
use std::fs::File;
use anyhow::{Context, Result};

#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path).
        with_context(|| format!("Could not open file `{:?}`", &args.path))?;
    let mut reader = BufReader::new(f);

    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        let s = line
            .with_context(|| format!("Could not readline `{}` in file `{:?}`", count, &args.path))?;
        println!("{}", &s);
        if s.contains(&args.pattern) {
            break;
        }
    }

    Ok(())
}
