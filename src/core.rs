use git2::Repository as GitRepository;
use std::fs::create_dir_all;
use std::path::Path;

const TMP_DIR: &str = "/tmp";
const AUR_URL: &str = "https://aur.archlinux.org/";

pub trait Repository {
    fn download(&self);
    fn install(&self);
    fn delete_from_tmp(&self);
}

pub fn download_package(repository: &impl Repository) {
    repository.download();
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

pub fn clone_package(package: &str) {
    let repo_url: String = format!("{}/{}.git", AUR_URL, package);
    let package_dir: String = format!("{}/{}", tmp_path(), package);
    match GitRepository::clone(repo_url.as_str(), &package_dir) {
        Ok(_) => println!("Cloned {}", package_dir),
        Err(_) => {
            println!("Failed to clone {}", package_dir);
            std::process::exit(1);
        }
    }
}

pub fn install_packages(repository: &impl Repository) {
    repository.install();
}

pub fn delete_from_tmp_packages(repository: &impl Repository) {
    repository.delete_from_tmp();
}
