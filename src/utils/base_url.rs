use std::env;
use once_cell::sync::Lazy;

pub struct BaseUrl;

impl BaseUrl {
    pub fn get() -> String {
        BASE_URL.clone()
    }
}

pub static BASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("BASE_URL").unwrap_or_else(|_| String::from("http://localhost:3000"))
});