pub trait LogLine: erased_serde::Serialize {
    fn table_name(&self) -> &'static str;
}
