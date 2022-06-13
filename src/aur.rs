use super::core::{clone_package, Repository};
use crate::core::{search_package_aur, tmp_path, Package};
use ansi_term::Style;
use std::process::{Command, Stdio};
use std::thread;

pub struct Aur {
    pub packages: Vec<String>,
}

impl Repository for Aur {
    fn search(&self) {
        let packages: Vec<String> = self.packages.clone();

        let mut results: Vec<Package> = Vec::new();

        for package in packages {
            results.append(&mut search_package_aur(&package));
        }

        if results.is_empty() {
            println!("No packages found.");
        }

        results.sort_by(|a, b| a.PackageBase.cmp(&b.PackageBase));

        for result in results {
            let maintainer = match result.Maintainer {
                Some(maintainer) => maintainer,
                None => "Unknown".to_string(),
            };
            println!(
                "pkg: {} - v: {}\nmaintainer: {}",
                Style::new().bold().paint(result.Name),
                Style::new().bold().paint(result.Version),
                Style::new().bold().paint(maintainer),
            );
        }
    }

    fn download(&self) {
        let packages: Vec<String> = self.packages.clone();
        println!("Downloading packages from AUR...");
        println!("{:?}", &packages);

        let mut handles = vec![];

        for package in packages {
            let handle = thread::spawn(move || {
                println!("Downloading {}...", package);

                clone_package(&package);

                println!("{} downloaded!", package);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn install(&self) {
        let packages: Vec<String> = self.packages.clone();

        println!("Installing packages from AUR...");

        for package in packages {
            let mut makepkg_cmd = Command::new("makepkg");
            let package_dir = format!("{}/{}", tmp_path(), package);
            makepkg_cmd.current_dir(package_dir);
            makepkg_cmd
                .arg("-sir")
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());

            match makepkg_cmd.output() {
                Ok(_) => println!("Installed {} succesfully", package),
                Err(e) => println!("There was an error {:?}", e),
            }
        }
    }

    fn delete_from_tmp(&self) {
        let packages: Vec<String> = self.packages.clone();

        for package in packages {
            let mut makepkg_cmd = Command::new("rm");
            makepkg_cmd.current_dir(tmp_path());
            makepkg_cmd
                .arg("-rf")
                .arg(&package)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());

            match makepkg_cmd.output() {
                Ok(_) => (),
                Err(e) => println!("There was an error {:?}", e),
            }
        }
    }
}
