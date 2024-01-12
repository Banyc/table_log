use std::sync::Mutex;

pub use logger::Logger;
use once_cell::sync::Lazy;
pub use record::LogRecord;

mod logger;
mod record;

pub static GLOBAL_LOG: Lazy<Mutex<GlobalLog>> = Lazy::new(|| Mutex::new(GlobalLog::new()));

pub fn log<'erased, R: LogRecord<'erased>>(record: &'erased R) {
    let mut log = GLOBAL_LOG.lock().unwrap();
    log.log(record);
}

pub struct GlobalLog {
    logger: Option<Box<dyn Logger + Send + Sync>>,
}
impl GlobalLog {
    pub fn new() -> Self {
        Self { logger: None }
    }

    pub fn try_register(
        &mut self,
        logger: Box<dyn Logger + Send + Sync>,
    ) -> Option<Box<dyn Logger + Send + Sync>> {
        if self.logger.is_none() {
            return Some(logger);
        }
        self.logger = Some(logger);
        None
    }

    pub fn register(&mut self, logger: Box<dyn Logger + Send + Sync>) {
        self.logger = Some(logger);
    }

    pub fn log<'erased, R: LogRecord<'erased>>(&mut self, record: &'erased R) {
        let Some(logger) = &mut self.logger else {
            return;
        };
        logger.log(record);
    }

    pub fn flush(&mut self) {
        if let Some(logger) = &mut self.logger {
            logger.flush();
        }
    }
}
impl Default for GlobalLog {
    fn default() -> Self {
        Self::new()
    }
}
