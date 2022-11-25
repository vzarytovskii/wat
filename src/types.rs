use clap::Parser;

use eyre::Report;
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::path::PathBuf;

// Cli is a struct that holds the command line arguments.
// Pretty simple for time being, since we only require one argume - a path to a file we want to analyze.
#[derive(Parser, Default, Debug)]
pub(crate) struct Cli {
    #[clap(value_parser, help = "Path to the file", value_hint = clap::ValueHint::FilePath, value_name = "PATH")]
    pub(crate) file_path: PathBuf,
}

pub(crate) struct FileView<'a> {
    pub(crate) path: &'a PathBuf,
    pub(crate) view: Mmap,
}

impl<'a> FileView<'a> {
    pub(crate) fn new(path: &'a PathBuf) -> Result<Self, Report> {
        Ok(Self {
            path,
            view: unsafe { MmapOptions::new().map(&File::open(path)?)? },
        })
    }
}
