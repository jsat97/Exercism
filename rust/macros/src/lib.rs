#[macro_export]
macro_rules! hashmap {
    ($(,)+) => 
    {{
         assert!(false);
     }};
    ($($k:expr => $v:expr),* $(,)?) =>
    {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($k, $v);
         ) *
        map
    }};
}
