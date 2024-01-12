pub trait LogRecord<'erased>: erased_serde::Serialize + 'erased {
    fn table_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn assert_ser(_s: impl serde::Serialize) {}

    #[allow(dead_code)]
    fn test_serde(record: &dyn LogRecord) {
        assert_ser(record as &dyn erased_serde::Serialize);
    }
}
