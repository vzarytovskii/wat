#![feature(seek_stream_len)]
#![feature(buf_read_has_data_left)]
#![feature(test)]

mod tests;
mod types;

use clap::Parser;
use eyre::Report;
use human_panic::setup_panic;
use types::{Cli, FileView};

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup_panic!();
    color_eyre::install()?;

    let cli = Cli::parse();

    let file_info = FileView::new(&cli.file_path)?;

    println!("Path: {}", file_info.path.display());
    println!("Len: {}", file_info.view.len());

    Ok(())
}
