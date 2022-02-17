use super::core::Repository;

pub struct Aur;
impl Repository for Aur {
    fn download(&self) {
        println!("downloading from aur");
    }
}
