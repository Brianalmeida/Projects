#![allow(unused)]

use std::result;
use clap::Parser;
use anyhow::{Context, Result};

// Search for a pattern in a file and display the lines.
#[derive(Parser)]
struct Cli {

    // The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
   let path = "test.txt";
   let content = std::fs::read_to_string(path)
       .with_context(|| format!("could not read file '{}'", path))?;
    println!("file content: {}", content);
    Ok(())
}
