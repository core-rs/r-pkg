use pkg::{aur::Aur, core, pacman::Pacman};

pub fn run() {
    let aur = Aur {};
    let pacman = Pacman {};

    core::download_package(&aur);
    core::download_package(&pacman);
}

fn main() {
    run();
}
