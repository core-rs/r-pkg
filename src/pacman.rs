use super::core::Repository;

pub struct Pacman;
impl Repository for Pacman {
    fn download(&self) {
        println!("downloading from pacman");
    }

    fn install(&self) {
        println!("installing from pacman");
    }

    fn delete_from_tmp(&self) {
        println!("deleting from pacman");
    }
}
