pub trait LogRecord<'erased>: erased_serde::Serialize + 'erased {
    fn table_name(&self) -> &'static str;
}

/// Make a reference of `T` of `erased_serde::Serialize` to also be `serde::Serialize`
///
/// So that you can put the reference of `T` to anywhere that requires `serde::Serialize`
pub struct SerWrap<'erased, T: ?Sized>(pub &'erased T);
impl<'erased, T> serde::Serialize for SerWrap<'erased, T>
where
    T: ?Sized + erased_serde::Serialize + 'erased,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.0, serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn assert_ser(_s: impl serde::Serialize) {}

    #[allow(dead_code)]
    fn test_serde(record: &dyn LogRecord) {
        let record = SerWrap(record);
        assert_ser(record);
    }
}
