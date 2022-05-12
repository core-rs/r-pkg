use super::core::Repository;
use std::thread;

pub struct Aur {
    pub packages: Vec<String>,
}

impl Repository for Aur {
    fn download(&self) {
        let packages = self.packages.clone();
        println!("Downloading packages from AUR...");
        println!("{:?}", &packages);

        let mut handles = vec![];

        for package in packages {
            let handle = thread::spawn(move || {
                println!("Downloading {}...", package);

                thread::sleep(std::time::Duration::from_millis(1000));

                println!("{} downloaded!", package);

                thread::sleep(std::time::Duration::from_millis(5000));

                println!("{} installed!", package);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
