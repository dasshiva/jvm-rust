use log::{Record, Level, Metadata};

struct Logger;
static LOGGER: Logger = Logger;

pub fn init() {
  log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }
  
  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{}: {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}