use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

mod error;
use error::{FileLoggerError, Result as FileLoggerResult};
// This logger will write logs into a file on disk
pub struct FileLogger {
    level: Level,
    writer: RwLock<BufWriter<File>>,
}
impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Check if the logger is enabled for a certain log level
        // Here, you could also add own custom filtering based on targets on regex.
        self.level >= metadata.level()
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self
                .writer
                .write()
                .expect("Failed to unlock log file writer in write mode");
            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).expect(
                "Failed to generate timestamp: This system is operation before the unix epoch",
            );
            // write the log into the buffer
            write!(
                writer,
                "{} {} at {}: {}\n",
                record.level(),
                timestamp.as_secs(),
                record.target(),
                record.args()
            )
            .expect("Failed to unlock log file write in write mode");
        }
        self.flush();
    }
    fn flush(&self) {
        self.writer
            .write()
            .expect("Failed to unlock log file writer in write mode")
            .flush()
            .expect("Failed to flush log file writer!");
    }
}
impl FileLogger {
    pub fn init(level: Level, filename: &str) -> FileLoggerResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(filename)?;
        let mut writer = RwLock::new(BufWriter::new(file));
        let logger = FileLogger { level, writer };
        log::set_max_level(level.to_level_filter());
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}
