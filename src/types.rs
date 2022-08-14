use clap::Parser;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tabled::Tabled;

#[derive(Parser, Default, Debug)]
pub struct Cli {
    #[clap(takes_value = true, value_parser, help = "Path to the file", value_hint = clap::ValueHint::FilePath, value_name = "PATH")]
    pub file_path: PathBuf,
}

pub struct FileInfo<'a> {
    pub path: &'a PathBuf,
    reader: BufReader<File>,
}

impl<'a> FileInfo<'a> {
    pub(crate) fn new(path: &'a PathBuf) -> Self {
        Self {
            path,
            reader: File::open(path).map(BufReader::new).unwrap(),
        }
    }

    pub(crate) fn can_read(&'a mut self) -> bool {
        self.reader.has_data_left().unwrap()
    }
}

#[derive(Tabled)]
pub struct FileData<'a> {
    #[tabled(rename = "File Name")]
    pub file_name: &'a str,
    #[tabled(rename = "File Path")]
    pub file_path: &'a str,
    #[tabled(rename = "File Type")]
    pub file_type: &'a str,
    #[tabled(rename = "Can read")]
    pub can_read: bool,
    #[tabled(rename = "Parsers")]
    pub known_parsers: &'a str,
}
