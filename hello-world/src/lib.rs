#![allow(dead_code)]
pub fn hello(x: Option <&str> ) -> String {
    match x {
        Some(ref x) => format!("Hello, {}!", x),
        None => "Hello, World!".to_string(),
    }
}
