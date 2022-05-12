use pkg::{aur::Aur, core};

fn run() {
    let packages: Vec<String> = vec![
        "datagrip".to_string(),
        "remote-desktop-manager".to_string(),
        "expressvpn".to_string(),
        "aws-cdk".to_string(),
    ];
    let aur = Aur {
        packages
    };

    core::download_package(&aur);
}

fn main() {
    run();
}
