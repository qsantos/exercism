#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {{
        let mut tmp = ::std::collections::HashMap::new();
        $( tmp.insert($key, $value); )*
        tmp
    }};
    ( $( $key:expr => $value:expr ),+ , ) => {{
        $crate::hashmap![ $( $key => $value ),+ ]
    }}
}
