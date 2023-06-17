use std::{
    fs,
    io::{self, Write},
    process::Command,
    time::Instant,
};
use tempfile::TempDir;

/// Returns the location of the `data` directory inside the crates.io database dump.
///
/// # Arguments
/// * `temp_dir` - the `TempDir` in which to search for the directory.
pub fn get_data_path(temp_dir: &TempDir) -> Option<String> {
    for dir_entry in fs::read_dir(temp_dir.path()).expect("Unable to read temporary directory") {
        let dir_entry = dir_entry.unwrap();
        if let Ok(file_type) = dir_entry.file_type() {
            // find the folder containing the dump results
            if file_type.is_dir() {
                let mut data_path = dir_entry.path();
                // append data to the path
                data_path.push("data");
                return Some(String::from(
                    data_path
                        .as_path()
                        .to_str()
                        .expect("Data path is not valid UTF-8"),
                ));
            }
        }
    }

    None
}

/// Load the crates.io data dump into a temporary directory.
///
/// Returns the temporary directory containing the dump data.
pub fn fetch_data() -> TempDir {
    println!("Fetching data...");
    let fetch_start = Instant::now();
    let temp_dir = TempDir::new().unwrap();
    let tgz_path = temp_dir.path().join("crates_data.tar.gz");
    let tgz_path_name = tgz_path
        .as_path()
        .to_str()
        .expect("Tarball path not valid UTF-8");

    println!("Downloading tarballed database dump...");
    let download_start = Instant::now();
    let curl_output = Command::new("curl")
        .arg("https://cloudfront-static.crates.io/db-dump.tar.gz")
        .arg("-o")
        .arg(tgz_path_name)
        .output()
        .expect("Unable to fetch Crates database dump");
    io::stdout().write_all(&curl_output.stdout).unwrap();
    io::stderr().write_all(&curl_output.stderr).unwrap();
    println!(
        "Downloaded tarballed database dump in {} seconds.",
        download_start.elapsed().as_secs_f64()
    );

    println!("Unpacking tarballed database dump...");
    let unpack_start = Instant::now();

    // exclude files not included in loading process
    Command::new("tar")
        .arg("-xzf")
        .arg(&tgz_path)
        .arg("--exclude")
        .arg("*/badges.csv")
        .arg("--exclude")
        .arg("*/crate_owners.csv")
        .arg("--exclude")
        .arg("*/metadata.csv")
        .arg("--exclude")
        .arg("*/reserved_crate_names.csv")
        .arg("--exclude")
        .arg("*/teams.csv")
        .arg("--exclude")
        .arg("*/users.csv")
        .arg("--exclude")
        .arg("*/version_*.csv")
        .arg("--exclude")
        .arg("*.sql")
        .arg("--exclude")
        .arg("*.json")
        .arg("--exclude")
        .arg("*.md")
        .arg("-C")
        .arg(temp_dir.path().to_str().unwrap())
        .output()
        .expect("Unable to fetch Crates database dump");

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

/// Cleans up a temporary directory.
///
/// # Arguments
/// * `temp_dir` - the `TempDir` to clean up.
pub fn clean_tempdir(temp_dir: TempDir) {
    println!("Cleaning up temporary files and directories...");
    let clean_start = Instant::now();
    temp_dir
        .close()
        .unwrap_or_else(|_| panic!("Unable to close temporary directory"));
    println!(
        "Temporary files and directories removed in {} seconds.",
        clean_start.elapsed().as_secs_f64()
    );
}
