use super::core::Repository;

pub struct Aur;
impl Repository for Aur {
    fn search(&self, package: &str) {
        println!("searching for {} in aur", package);
    }

    fn download(&self, package: &str) -> String {
        format!("downloading {} from aur", package)
    }

    fn install(&self, package: &str) -> String {
        format!("installing {} from aur", package)
    }

    fn remove(&self, package: &str) -> bool {
        println!("removing {} from aur", package);
        true
    }
}
