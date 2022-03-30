use pkg::{aur::Aur, core, pacman::Pacman};

pub fn run() {
    let aur = Aur {};
    let pacman = Pacman {};

    println!("{}", core::download_package(&aur, "pacman"));
    core::search_package(&pacman, "pacman");
}

fn main() {
    run();
}
