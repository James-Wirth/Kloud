use std::collections::HashMap;

pub struct FileStorage {
    files: HashMap<String, Vec<u8>>,
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: &str, data: &[u8]) {
        self.files.insert(name.to_string(), data.to_vec());
    }

    pub fn get_file(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.get(name)
    }
}
