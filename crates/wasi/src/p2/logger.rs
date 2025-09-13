
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    FATAL
}

pub trait Logger {
    fn log(&mut self, log_level: LogLevel, log: String);
}

pub struct DummyLogger {}

impl Logger for DummyLogger {
    fn log(&mut self, _log_level: LogLevel, _log: String) {
        
    }
}
