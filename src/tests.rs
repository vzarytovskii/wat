#[cfg(test)]
mod tests {

    extern crate test;
    use crate::FileInfo;
    use std::path::PathBuf;

    use test::Bencher;

    #[test]
    fn can_open_file() {
        let file = PathBuf::from("./Cargo.toml");
        let mut file_info = FileInfo::new(&file);
        let can_read = file_info.can_read();

        assert_eq!(true, can_read);
    }

    #[bench]
    fn open_file_and_read_len(b: &mut Bencher) {
        b.iter(|| {
            let file = PathBuf::from("./Cargo.toml");
            let mut file_info = FileInfo::new(&file);
            let can_read = file_info.can_read();
            assert_eq!(true, can_read);
        });
    }
}
