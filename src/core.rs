#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub Description: Option<String>,
    pub FirstSubmitted: i64,
    pub ID: i64,
    pub LastModified: i64,
    pub Maintainer: Option<String>,
    pub Name: String,
    pub NumVotes: i64,
    pub OutOfDate: Option<i64>,
    pub PackageBase: String,
    pub PackageBaseID: i64,
    pub Popularity: f64,
    pub URL: Option<String>,
    pub URLPath: String,
    pub Version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageResult {
    resultcount: u32,
    version: u32,
    results: Vec<Package>,
}

const TMP_DIR: &str = "/tmp";
const AUR_URL: &str = "https://aur.archlinux.org/";

pub trait Repository {
    fn search(&self);
    fn download(&self);
    fn install(&self);
    fn delete_from_tmp(&self);
}

pub fn download_package(repository: &impl Repository) {
    repository.download();
}

pub fn search_package(repository: &impl Repository) {
    repository.search();
}

pub fn tmp_path() -> String {
    vec![
        Path::new(TMP_DIR).to_str().unwrap().to_owned(),
        ".pkg".to_owned(),
    ]
    .join("/")
}

fn create_tmp_dir() {
    let tmp_path = tmp_path();
    let _tmp_dir = Path::new(&tmp_path);
    if !_tmp_dir.exists() {
        match create_dir_all(&tmp_path) {
            Ok(_) => println!("Created {}", tmp_path),
            Err(_) => {
                println!("Failed to create {}", tmp_path);
                std::process::exit(1);
            }
        };
    }
}

pub fn init() {
    create_tmp_dir();
}

pub fn search_package_aur(package: &str) -> Vec<Package> {
    let aur_url = format!("{}rpc?v=5&type=search&arg={}", AUR_URL, package);
    let resp = match reqwest::blocking::get(&aur_url) {
        Ok(resp) => resp.json::<PackageResult>(),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let package_result = match resp {
        Ok(package_result) => package_result,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    package_result.results
}

pub fn clone_package(package: &str) {
    let repo_url: String = format!("{}/{}.git", AUR_URL, package);
    let package_dir: String = format!("{}/{}", tmp_path(), package);
    let mut clone_cmd = Command::new("git");
    let status = clone_cmd
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .arg("clone")
        .arg("-q")
        .arg("--depth=1")
        .arg(repo_url.as_str())
        .arg(package_dir)
        .status()
        .expect("Failed to execute git clone");

    match status.code() {
        Some(0) => println!("{} cloned!", package),
        Some(_) => println!("Failed to clone {}", package),
        None => println!("Failed to clone {}", package),
    }
}

pub fn install_packages(repository: &impl Repository) {
    repository.install();
}

pub fn delete_from_tmp_packages(repository: &impl Repository) {
    repository.delete_from_tmp();
}
