use std::path::PathBuf;

macro_rules! exit {
    ($message:expr,$code:expr) => {
        eprintln!("{}", format!($message));
        std::process::exit($code);
    };
}

pub(crate) fn verify_file_path(file: &PathBuf) {
    let path_display = file.display();
    if !file.exists() {
        exit!("File does not exist: '{path_display}'", exitcode::NOINPUT);
    }

    if file.is_dir() {
        exit!("'{path_display}' is a directory.", exitcode::NOINPUT);
    }
}
