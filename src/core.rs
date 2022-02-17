pub trait Repository {
    fn download(&self);
}

pub fn download_package(repository: &impl Repository) {
    repository.download();
}
