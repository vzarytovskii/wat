#![feature(seek_stream_len)]
#![feature(buf_read_has_data_left)]
#![feature(test)]

mod tests;
mod types;
mod utils;

extern crate exitcode;

use clap::Parser;
use human_panic::setup_panic;
use std::process::ExitCode;
use tabled::{Style, Table};
use types::{Cli, FileData, FileInfo};
use utils::verify_file_path;

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
