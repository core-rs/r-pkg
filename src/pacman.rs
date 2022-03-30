use super::core::Repository;

pub struct Pacman;
impl Repository for Pacman {
    fn search(&self, package: &str) {
        println!("searching for {} in pacmam", package);
    }

    fn download(&self, package: &str) -> String {
        format!("downloading {} from pacmam", package)
    }

    fn install(&self, package: &str) -> String {
        format!("installing {} from pacmam", package)
    }

    fn remove(&self, package: &str) -> bool {
        println!("removing {} from pacmam", package);
        true
    }
}
