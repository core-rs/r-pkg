use super::core::Repository;

pub struct Pacman;
impl Repository for Pacman {
    fn download(&self) {
        println!("downloading from pacman");
    }
}
