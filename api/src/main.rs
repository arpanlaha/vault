// use git2::Repository;
use serde::Deserialize;
use std::path::Path;
// use tempfile::{tempdir, TempDir};
// use semver_parser::version;
use arangors::Connection;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use tokio::runtime::Runtime;

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

#[derive(Deserialize, Debug)]
struct Category {
    category: String,
    description: String,
    id: usize,
    path: String,
    slug: String,
}

fn traverse_dir(path: &Path, crates: &mut HashMap<String, Crate>) -> io::Result<()> {
    if !path.to_str().unwrap().ends_with(".git") {
        for dir_entry in fs::read_dir(path)? {
            let dir_entry = dir_entry?;
            if dir_entry.file_type()?.is_dir() {
                traverse_dir(dir_entry.path().as_path(), crates)?;
            } else {
                for line in BufReader::new(File::open(dir_entry.path().as_path())?).lines() {
                    let deserialized: Result<Crate, _> = serde_json::from_str(&line?);
                    if let Ok(line_crate) = deserialized {
                        crates.insert(
                            format!("{}@{}", line_crate.name, line_crate.vers),
                            line_crate,
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn read_categories() -> io::Result<()> {
    for result in csv::Reader::from_reader(BufReader::new(File::open(Path::new(
        "../dump/data/categories.csv",
    ))?))
    .deserialize()
    {
        let record: Category = result?;
        println!("{:?}\n", record);
    }
    Ok(())
}

async fn connect_db() {
    let conn = Connection::establish_jwt(
        dotenv::var("ARANGODB_URI").unwrap().as_str(),
        "root",
        dotenv::var("ARANGODB_ROOT_PASSWORD").unwrap().as_str(),
    )
    .await
    .unwrap();
}

fn main() {
    dotenv::dotenv().unwrap();

    // let temp_dir: TempDir = tempdir().unwrap();
    // let path: &Path = temp_dir.path();
    // Repository::clone("https://github.com/rust-lang/crates.io-index.git", path).unwrap();
    println!("Hello, world!");
    // read_categories().unwrap();

    Runtime::new().unwrap().block_on(connect_db());

    // for dir_entry in fs::read_dir(Path::new("data")).unwrap() {
    //     let dir_entry = dir_entry
    // }

    // let mut crates: HashMap<String, Crate> = HashMap::new();

    // traverse_dir(Path::new("data"), &mut crates).unwrap();

    // println!("number of crates: {}", crates.len());

    // let example_path = Path::new("data/ac/ti/actix-web");
    // let file_contents = fs::read_to_string(example_path).unwrap();
    // for line in file_contents.split("\n").filter(|line| line.len() > 0) {
    //     // println!("size: {}", line.len());
    //     let line_crate: Crate = serde_json::from_str(&line).unwrap();
    //     println!("crate: {:?}\n", line_crate);
    // }
    // let example_file = File::open(example_path).unwrap();
    // temp_dir.close().unwrap();
}
