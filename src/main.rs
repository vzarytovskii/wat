#![feature(seek_stream_len)]
#![feature(buf_read_has_data_left)]
#![feature(test)]

mod analyzers;
mod types;

use clap::Parser;
use eyre::Report;
use human_panic::setup_panic;
use types::{Cli, FileView};

use crate::analyzers::analyze;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Report> {
    setup_panic!();
    color_eyre::install()?;

    let cli = Cli::parse();
    let file_info = FileView::new(&cli.file_path)?;

    // TODO(?): Move to some proper tui widget-based library (e.g. tui+tuirealm).
    analyze(&file_info).await?;

    Ok(())
}
