use curl::easy::Easy;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tar::Archive;
use tempfile::TempDir;

pub fn get_data_path(import_path: &String) -> Option<String> {
    for dir_entry in fs::read_dir(import_path).expect("Unable to read temporary directory") {
        let dir_entry = dir_entry.unwrap();
        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_dir() {
                let mut data_path = dir_entry.path();
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

fn change_permissions(file: &File, flags: u32) {
    let mut permissions = file
        .metadata()
        .expect("Unable to access file metadata")
        .permissions();
    permissions.set_mode(flags);
    file.set_permissions(permissions)
        .expect("Unable to modify permissions");
}

pub fn fetch_data() -> String {
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
    let import_path = String::from("/var/lib/neo4j/import");

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
        .unpack(&import_path)
        .expect("Unable to unpack database dump TAR archive");
    println!(
        "Unpacked database dump TAR archive in {} seconds.",
        unpack_start.elapsed().as_secs_f64()
    );

    // TODO: change file permissions

    let data_path = get_data_path(&import_path).unwrap();
    let data_parent_path = Path::new(&data_path)
        .parent()
        .expect("Unable to find data path parent");

    change_permissions(
        &File::open(&data_parent_path).expect(
            format!(
                "Unable to open {}",
                &data_parent_path
                    .to_str()
                    .expect("Data path parent is not valid UTF-8")
            )
            .as_str(),
        ),
        0o777,
    );

    change_permissions(
        &File::open(&data_path).expect(format!("Unable to open {}", &data_path).as_str()),
        0o777,
    );

    for dir_entry in fs::read_dir(data_path).expect("Unable to read data path") {
        let dir_entry = dir_entry.unwrap();
        let file = File::open(dir_entry.path()).expect(
            format!(
                "Unable to open {}",
                dir_entry
                    .path()
                    .as_path()
                    .to_str()
                    .expect("Data path is not valid UTF-8")
            )
            .as_str(),
        );
        change_permissions(&file, 0o644);
    }

    temp_dir.close().unwrap();

    println!(
        "Finished fetching data in {} seconds.",
        fetch_start.elapsed().as_secs_f64()
    );

    import_path
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
