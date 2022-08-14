extern crate exitcode;

use clap::Parser;
use human_panic::setup_panic;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::ExitCode;
use tabled::{Footer, Header, Tabled, Table, Style};

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

#[derive(Tabled)]
struct FileData<'a> {
    #[tabled(rename = "File Name")]
    file_name: &'a str,
    #[tabled(rename = "File Path")]
    file_path: &'a str,
    #[tabled(rename = "File Type")]
    file_type: &'a str,
    #[tabled(rename = "Parsers")]
    known_parsers: &'a str
}


macro_rules! exit {
    ($message:expr,$code:expr) => {
        eprintln!("{}", format!($message));
        std::process::exit($code);
    };
}

fn verify_file_path<'a>(file: &'a PathBuf) {
    let path_display = file.display();
    if !file.exists() {
        exit!("File does not exist: '{path_display}'", exitcode::NOINPUT);
    }

    if file.is_dir() {
        exit!("'{path_display}' is a directory.", exitcode::NOINPUT);
    }
}

fn main() -> ExitCode {
    setup_panic!();

    let cli = Cli::parse();
    let file = cli.file_path;

    verify_file_path(&file);

    let file_info = FileInfo::new(&file);

    let data = vec![
        FileData {
            file_name: file_info.path.file_name().unwrap().to_str().unwrap(),
            file_path: file_info.path.to_str().unwrap(),
            file_type: "Unknown",
            known_parsers: "GenericBinaryParser, EntropyAnalyzer"
        }
    ];

    let table =
        Table::new(data)
        .with(Style::rounded())
        //.with(Header("File analysis results"))
        //.with(Footer("Time spent: 0.00s"))
        .to_string();

    println!("{}", table);

    ExitCode::SUCCESS
}
