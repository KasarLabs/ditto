#[macro_export]
macro_rules! map {
    // Match a comma-separated list of key-value pairs.
    { $( $key:expr => $value:expr ),* $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
}