use log::{self, Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

struct Logger {}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => "\u{1B}[31m",
            Level::Warn => "\u{1B}[93m",
            Level::Info => "\u{1B}[34m",
            Level::Debug => "\u{1B}[32m",
            Level::Trace => "\u{1B}[90m",
        };
        println!(
            "{}[{:>5}] {}{}",
            color,
            record.level(),
            record.args(),
            "\u{1B}[0m"
        );
    }
    fn flush(&self) {}
}

pub fn init(max_level: LevelFilter) -> Result<(), SetLoggerError> {
    static LOGGER: Logger = Logger {};
    log::set_logger(&LOGGER).map(|()| log::set_max_level(max_level))
}
