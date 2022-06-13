use pkg::{aur::Aur, core};

fn run() {
    let packages: Vec<String> = vec![
        "slack-desktop".to_string(),
        "visual-studio-code-insiders-bin".to_string(),
    ];
    let aur = Aur { packages };

    core::init();

    core::download_package(&aur);

    core::install_packages(&aur);

    core::delete_from_tmp_packages(&aur);
}

fn main() {
    run();
}
