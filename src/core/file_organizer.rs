use crate::core::error::SeokjinError;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

pub struct FileOrganizer {
    rules: HashMap<String, String>,
}

impl FileOrganizer {
    pub fn new() -> Self {
        FileOrganizer {
            rules: HashMap::new(),
        }
    }

    pub fn organize_directory(&self, path: &Path) -> Result<(), SeokjinError> {
        if !path.is_dir() {
            return Err(SeokjinError::InvalidInput(format!("{} is not a directory", path.display())));
        }

        for entry in fs::read_dir(path).map_err(|e| SeokjinError::IOError(e.to_string()))? {
            let entry = entry.map_err(|e| SeokjinError::IOError(e.to_string()))?;
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    if let Some(folder) = self.rules.get(extension.to_str().unwrap_or("")) {
                        let target_dir = path.join(folder);
                        fs::create_dir_all(&target_dir).map_err(|e| SeokjinError::IOError(e.to_string()))?;
                        let new_path = target_dir.join(file_path.file_name().unwrap());
                        fs::rename(&file_path, &new_path).map_err(|e| SeokjinError::IOError(e.to_string()))?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_rule(&mut self, extension: String, folder: String) {
        self.rules.insert(extension, folder);
    }

    pub fn remove_rule(&mut self, extension: &str) {
        self.rules.remove(extension);
    }
}
