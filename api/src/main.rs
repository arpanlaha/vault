// use git2::Repository;
use serde::Deserialize;
use std::path::Path;
// use tempfile::{tempdir, TempDir};
use std::fs;

#[derive(Deserialize, Debug)]
struct Crate {
    name: String,
    vers: String,
    deps: Vec<Dependency>,
}

#[derive(Deserialize, Debug)]
struct Dependency {
    name: String,
    req: String,
    kind: String,
}

fn main() {
    // let temp_dir: TempDir = tempdir().unwrap();
    // let path: &Path = temp_dir.path();
    // Repository::clone("https://github.com/rust-lang/crates.io-index.git", path).unwrap();
    println!("Hello, world!");

    let example_path = Path::new("data/ac/ti/actix-web");
    let file_contents = fs::read_to_string(example_path).unwrap();
    for line in file_contents.split("\n").filter(|line| line.len() > 0) {
        // println!("size: {}", line.len());
        let line_crate: Crate = serde_json::from_str(&line).unwrap();
        println!("crate: {:?}\n", line_crate);
    }
    // let example_file = File::open(example_path).unwrap();
    // temp_dir.close().unwrap();
}
