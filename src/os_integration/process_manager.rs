use std::process::Command;
use crate::core::error::SeokjinError;

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager
    }

    pub fn open_application(&self, app_name: &str) -> Result<String, SeokjinError> {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "start", app_name])
                .spawn()
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg("-a")
                .arg(app_name)
                .spawn()
        } else {
            Command::new(app_name)
                .spawn()
        }.map_err(|e| SeokjinError::SystemError(format!("Failed to open {}: {}", app_name, e)))?;

        Ok(format!("Opened {}", app_name))
    }

    pub fn get_running_processes(&self) -> Result<Vec<String>, SeokjinError> {
        let (command, args) = if cfg!(target_os = "windows") {
            ("tasklist", vec!["/FO", "CSV", "/NH"])
        } else {
            ("ps", vec!["-e", "-o", "comm="])
        };

        let output = Command::new(command)
            .args(&args)
            .output()
            .map_err(|e| SeokjinError::SystemError(format!("Failed to get process list: {}", e)))?;

        let process_list = String::from_utf8_lossy(&output.stdout);
        
        Ok(process_list
            .lines()
            .map(|line| {
                if cfg!(target_os = "windows") {
                    line.split(',').next().unwrap_or("").trim_matches('"').to_string()
                } else {
                    line.trim().to_string()
                }
            })
            .filter(|s| !s.is_empty())
            .collect())
    }
}
