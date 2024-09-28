use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use crate::core::error::SeokjinError;

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        FileSystem
    }

    pub fn read_file(&self, path: &str) -> Result<String, SeokjinError> {
        let mut file = File::open(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to open file: {}", e)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to read file: {}", e)))?;
        Ok(contents)
    }

    pub fn write_file(&self, path: &str, contents: &str) -> Result<(), SeokjinError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to create/open file: {}", e)))?;
        file.write_all(contents.as_bytes())
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to write to file: {}", e)))?;
        Ok(())
    }

    pub fn append_to_file(&self, path: &str, content: &str) -> Result<(), SeokjinError> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to open file for appending: {}", e)))?;
        file.write_all(content.as_bytes())
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to append to file: {}", e)))?;
        Ok(())
    }

    pub fn delete_file(&self, path: &str) -> Result<(), SeokjinError> {
        fs::remove_file(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to delete file: {}", e)))?;
        Ok(())
    }

    pub fn create_directory(&self, path: &str) -> Result<(), SeokjinError> {
        fs::create_dir_all(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to create directory: {}", e)))?;
        Ok(())
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, SeokjinError> {
        let entries = fs::read_dir(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to read directory: {}", e)))?;
        let mut file_names = Vec::new();
        for entry in entries {
            let entry = entry
                .map_err(|e| SeokjinError::FileSystemError(format!("Failed to read directory entry: {}", e)))?;
            let file_name = entry.file_name().into_string()
                .map_err(|_| SeokjinError::FileSystemError("Invalid file name".to_string()))?;
            file_names.push(file_name);
        }
        Ok(file_names)
    }

    pub fn file_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn is_directory(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }

    pub fn get_file_size(&self, path: &str) -> Result<u64, SeokjinError> {
        let metadata = fs::metadata(path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to get file metadata: {}", e)))?;
        Ok(metadata.len())
    }

    pub fn rename_file(&self, old_path: &str, new_path: &str) -> Result<(), SeokjinError> {
        fs::rename(old_path, new_path)
            .map_err(|e| SeokjinError::FileSystemError(format!("Failed to rename file: {}", e)))?;
        Ok(())
    }
}
