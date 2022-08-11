extern crate exitcode;

use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(takes_value = true, value_parser, help = "Path to the file", value_hint = clap::ValueHint::FilePath, value_name = "PATH")]
    file_path: PathBuf,
}

struct FileInfo<'a> {
    path: &'a PathBuf,
    reader: BufReader<File>,
}

impl<'a> FileInfo<'a> {
    fn new(path: &'a PathBuf) -> Self {
        Self {
            path,
            reader: File::open(path).map(BufReader::new).unwrap(),
        }
    }
}

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

fn main() -> ExitCode {
    let cli = Cli::parse();
    let file = cli.file_path;

    verify_file_path(&file);

    let file_info = FileInfo::new(&file);

    println!("{:?}", file_info.path);

    ExitCode::SUCCESS
}
