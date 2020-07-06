use curl::easy::Easy;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;
use tar::Archive;
use tempfile::TempDir;

pub fn get_data_path(temp_dir: &TempDir) -> Option<String> {
    for dir_entry in fs::read_dir(temp_dir.path()).expect("Unable to read temporary directory") {
        let dir_entry = dir_entry.unwrap();
        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_dir() {
                return Some(String::from(
                    dir_entry
                        .path()
                        .as_path()
                        .to_str()
                        .expect("Data path is not valid UTF-8"),
                ));
            }
        }
    }

    None
}

pub fn fetch_data() -> TempDir {
    println!("Fetching data...");
    let fetch_start = Instant::now();
    let temp_dir = TempDir::new().unwrap();
    let tgz_path = temp_dir.path().join("crates_data.tar.gz");
    let tgz_path_name = tgz_path
        .as_path()
        .to_str()
        .expect("Tarball path not valid UTF-8");
    let mut tgz_file = File::create(tgz_path.as_path())
        .expect(format!("Unable to create {}", tgz_path_name).as_str());

    println!("Downloading tarballed database dump...");
    let download_start = Instant::now();
    let mut curl = Easy::new();
    curl.url("https://static.crates.io/db-dump.tar.gz").unwrap();
    curl.write_function(move |data| Ok(tgz_file.write(data).unwrap()))
        .unwrap();
    curl.perform().unwrap();
    println!(
        "Tarballed database dump downloaded in {} seconds.",
        download_start.elapsed().as_secs_f64()
    );

    println!("Unzipping tarballed database dump...");
    let unzip_start = Instant::now();
    let tar = GzDecoder::new(
        File::open(tgz_path_name).expect(format!("Unable to open {}", tgz_path_name).as_str()),
    );
    println!(
        "Unzipped tarballed database dump into TAR archive in {} seconds.",
        unzip_start.elapsed().as_secs_f64()
    );

    println!("Unpacking database dump TAR archive...");
    let unpack_start = Instant::now();
    Archive::new(tar)
        .unpack(temp_dir.path())
        .expect("Unable to unpack database dump TAR archive");
    println!(
        "Unpacked database dump TAR archive in {} seconds.",
        unpack_start.elapsed().as_secs_f64()
    );

    println!(
        "Finished fetching data in {} seconds.",
        fetch_start.elapsed().as_secs_f64()
    );
    temp_dir
}

pub fn clean_tempdir(temp_dir: TempDir) {
    println!("Cleaning up temporary files and directories...");
    let clean_start = Instant::now();
    temp_dir
        .close()
        .expect("Unable to close temporary directory");
    println!(
        "Temporary files and directories removed in {} seconds.",
        clean_start.elapsed().as_secs_f64()
    );
}
