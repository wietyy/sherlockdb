use std::fs;


pub struct Database {
    path: str,
}

impl Database {
    pub fn set() {
        
    }

    pub fn get(&self, id: &str) -> String {
        let filepath = format!("{}/{}", &self.path, id);
        let contents =  fs::read_to_string(filepath.as_str());
        return contents.unwrap();
    }

    pub fn delete() {

    }
}