use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct OpenedFile {
    pub needs_reloading: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
}

impl OpenedFile {
    pub fn new(initial_file: Option<PathBuf>) -> Self {
        Self {
            needs_reloading: initial_file,
            current_file: None,
        }
    }

    pub fn request_open<P>(&mut self, path: P)
        where P: AsRef<Path>
    {
        self.needs_reloading = Some(PathBuf::from(path.as_ref()));
    }

    pub fn check_load(&mut self) -> Option<PathBuf> {
        if let Some(path) = self.needs_reloading.clone() {
            self.current_file.clone_from(&self.needs_reloading);
            self.needs_reloading = None;

            Some(path.clone())
        } else {
            None
        }
    }
}
