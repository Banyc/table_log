use crate::log_line::LogLine;

pub trait Logger {
    fn log(&mut self, value: &dyn LogLine);
}
