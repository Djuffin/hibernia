use log::{LevelFilter, Metadata, Record};
use std::sync::Once;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
static INIT: Once = Once::new();

pub fn init(trace: bool) {
    INIT.call_once(|| {
        let _ = log::set_logger(&LOGGER);
    });
    log::set_max_level(if trace { LevelFilter::Trace } else { LevelFilter::Info });
}
