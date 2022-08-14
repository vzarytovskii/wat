#![feature(seek_stream_len)]
#![feature(buf_read_has_data_left)]
#![feature(test)]

mod tests;
mod types;

extern crate exitcode;

use clap::Parser;
use human_panic::setup_panic;
use std::path::PathBuf;
use std::process::ExitCode;
use tabled::{Style, Table};
use types::{Cli, FileData, FileInfo};

macro_rules! exit {
    ($message:expr,$code:expr) => {
        eprintln!("{}", format!($message));
        std::process::exit($code);
    };
}

fn verify_file_path(file: &PathBuf) {
    let path_display = file.display();
    if !file.exists() {
        exit!("File does not exist: '{path_display}'", exitcode::NOINPUT);
    }

    if file.is_dir() {
        exit!("'{path_display}' is a directory.", exitcode::NOINPUT);
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    setup_panic!();

    let cli = Cli::parse();
    let file = cli.file_path;

    verify_file_path(&file);

    let mut file_info = FileInfo::new(&file);

    let data = vec![FileData {
        file_name: file_info.path.file_name().unwrap().to_str().unwrap(),
        file_path: file_info.path.to_str().unwrap(),
        file_type: "Unknown",
        can_read: file_info.can_read(),
        known_parsers: "GenericBinaryParser, EntropyAnalyzer",
    }];

    let table = Table::new(data)
        .with(Style::rounded())
        //.with(Header("File analysis results"))
        //.with(Footer("Time spent: 0.00s"))
        .to_string();

    println!("{}", table);

    ExitCode::SUCCESS
}
