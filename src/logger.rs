use crate::record::LogRecord;

pub trait Logger {
    fn log(&mut self, value: &dyn LogRecord<'_>);
    fn flush(&mut self);
}
