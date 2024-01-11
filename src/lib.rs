use std::sync::Mutex;

pub use log_line::LogLine;
pub use logger::Logger;
use once_cell::sync::Lazy;

mod log_line;
mod logger;

pub static GLOBAL_LOG: Lazy<Mutex<GlobalLog>> = Lazy::new(|| Mutex::new(GlobalLog::new()));

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

    pub fn log<L: LogLine>(&mut self, value: &L) {
        let Some(logger) = &mut self.logger else {
            return;
        };
        logger.log(value);
    }
}
impl Default for GlobalLog {
    fn default() -> Self {
        Self::new()
    }
}
