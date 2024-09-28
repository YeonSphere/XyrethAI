use crate::core::error::SeokjinError;
use std::process::Command;
use battery::Manager;
use chrono::{Local, NaiveTime};

pub struct PlatformModule;

impl PlatformModule {
    pub fn new() -> Self {
        PlatformModule
    }

    pub fn set_alarm(&self, time: &str) -> Result<String, SeokjinError> {
        let parsed_time = NaiveTime::parse_from_str(time, "%H:%M")
            .map_err(|_| SeokjinError::InvalidInput("Invalid time format".to_string()))?;
        
        let now = Local::now().time();
        let duration = if parsed_time > now {
            parsed_time - now
        } else {
            (parsed_time + chrono::Duration::days(1)) - now
        };

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(duration.num_seconds() as u64));
            println!("ALARM: It's {}!", time);
            // Here you could add code to play a sound or show a notification
        });

        Ok(format!("Alarm set for {}", time))
    }

    pub fn open_app(&self, app_name: &str) -> Result<String, SeokjinError> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "start", app_name])
                .output()
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg("-a")
                .arg(app_name)
                .output()
        } else {
            Command::new(app_name)
                .output()
        };

        match output {
            Ok(_) => Ok(format!("Opened {}", app_name)),
            Err(e) => Err(SeokjinError::SystemError(format!("Failed to open {}: {}", app_name, e))),
        }
    }

    pub fn get_battery_level(&self) -> Result<String, SeokjinError> {
        let manager = Manager::new().map_err(|e| SeokjinError::SystemError(e.to_string()))?;
        let battery = manager.batteries().map_err(|e| SeokjinError::SystemError(e.to_string()))?
            .next()
            .ok_or_else(|| SeokjinError::SystemError("No battery found".to_string()))?
            .map_err(|e| SeokjinError::SystemError(e.to_string()))?;

        let percentage = battery.state_of_charge().value() * 100.0;
        Ok(format!("Battery level: {:.0}%", percentage))
    }
}
