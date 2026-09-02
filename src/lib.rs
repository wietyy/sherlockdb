use std::fs;
use std::path::PathBuf;

pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Database {
            path: PathBuf::from(path),
        }
    }

    pub fn set(&self, id: &str, contents: &str) -> Result<(), std::io::Error> {
        let filepath = self.path.join(id);
        fs::create_dir_all(&self.path)?;
        fs::write(filepath, contents)
    }

    pub fn get(&self, id: &str) -> Result<String, std::io::Error> {
        let filepath = self.path.join(id);
        fs::read_to_string(filepath)
    }

    pub fn delete(&self, id: &str) -> Result<(), std::io::Error> {
        let filepath = self.path.join(id);
        match fs::remove_file(filepath) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }
}