use crate::core::error::SeokjinError;
use std::fs;
use unicode_normalization::UnicodeNormalization;
use regex::Regex;

pub fn preprocess_data(file_path: &str) -> Result<Vec<String>, SeokjinError> {
    let content = fs::read_to_string(file_path).map_err(|e| SeokjinError::IOError(e.to_string()))?;
    let processed_data: Vec<String> = content
        .lines()
        .map(|line| {
            let normalized = line.nfkc().collect::<String>();
            let trimmed = normalized.trim();
            let lowercase = trimmed.to_lowercase();
            remove_special_characters(&lowercase)
        })
        .filter(|line| !line.is_empty())
        .collect();
    Ok(processed_data)
}

fn remove_special_characters(text: &str) -> String {
    let re = Regex::new(r"[^a-z0-9\s]").unwrap();
    re.replace_all(text, "").to_string()
}
