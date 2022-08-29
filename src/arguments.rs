#![allow(unused)]

use clap::Parser;
use indicatif::ProgressBar;
use std::io::{self, BufReader, BufRead, Read, Write};
use std::fs::{File, OpenOptions};
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

fn test_bufwriter() -> Result<()> {
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/text.txt")
        .with_context(|| format!("Could not open file `{:?}`", "src/text.txt"))?;
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(f); // optional: wrap that handle in buffer
    writeln!(handle, "foo: {}", 42); 
    writeln!(handle, "feed: {}", 45);
    Ok(())
}

fn progress_bar() {
    let pb = indicatif::ProgressBar::new(10);
    for i in 0..10 {
        test_bufwriter();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

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

    // Test progress bar and BufWriter code
    //progress_bar();

    // Test log crate
    

    Ok(())
}

#[test]
fn check_answer_validity() {
    assert_eq!(43, 43);
}
