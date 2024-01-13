#[test]
fn test_log() {
    #[derive(serde::Serialize)]
    struct Record {
        n: usize,
        s: String,
    }
    impl table_log::LogRecord<'_> for Record {
        fn table_name(&self) -> &'static str {
            "test"
        }
    }
    table_log::log!(&Record {
        n: 0,
        s: "0".to_string()
    });
    table_log::log!(&Record {
        n: 1,
        s: "1".to_string()
    });
}
