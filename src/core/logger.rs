use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

struct FileLogger {
    file: std::fs::File,
}

impl FileLogger {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename)?;
        Ok(FileLogger { file })
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            let log_entry = format!("{} - {} - {}\n", timestamp, record.level(), record.args());
            let _ = self.file.write_all(log_entry.as_bytes());
        }
    }

    fn flush(&self) {
        let _ = self.file.sync_all();
    }
}

static mut LOGGER: Option<FileLogger> = None;

pub fn init(filename: &str) -> Result<(), SetLoggerError> {
    unsafe {
        LOGGER = Some(FileLogger::new(filename).map_err(|_| SetLoggerError::CannotCreate)?);
        log::set_logger(LOGGER.as_ref().unwrap())
            .map(|()| log::set_max_level(LevelFilter::Debug))
    }
}
