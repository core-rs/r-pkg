pub trait Repository {
    fn download(&self, package: &str) -> String;
    fn install(&self, package: &str) -> String;
    fn search(&self, package: &str);
    fn remove(&self, package: &str) -> bool;
}

pub fn search_package(repository: &impl Repository, package: &str) {
    repository.search(package);
}

pub fn download_package(repository: &impl Repository, package: &str) -> String {
    repository.download(package)
}

pub fn install_package(repository: &impl Repository, package: &str) -> String {
    repository.install(package)
}

pub fn uninstall_package(repository: &impl Repository, package: &str) -> bool {
    repository.remove(package)
}
