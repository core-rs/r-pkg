use super::core::{clone_package, Repository};
use crate::core::tmp_path;
use std::process::{Command, Stdio};
use std::thread;

pub struct Aur {
    pub packages: Vec<String>,
}

impl Repository for Aur {
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
                Ok(_) => println!("Removed {} from tmp succesfully", package),
                Err(e) => println!("There was an error {:?}", e),
            }
        }
    }
}
