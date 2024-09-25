use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// The URL prefix to query WBM. The protocol is accepted but not neccessary.
    /// Example: www.example.com, www.example.com/path
    pub url: String,
    /// Match captures after this date.
    /// Valid formats: YYYY-MM-DD, YYYY-MM, YYYY
    pub from_date: String,
    /// Match captures before this date.
    /// Valid formats: YYYY-MM-DD, YYYY-MM, YYYY
    pub until_date: String,
    /// Set the output directory for the captures
    #[clap(short, long, default_value = "./")]
    pub output_directory: PathBuf,
    /// Verbose output logs if files are already downloaded
    #[clap(short, long)]
    pub verbose: bool,
}
