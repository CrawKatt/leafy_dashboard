use std::env;
use once_cell::sync::Lazy;

pub struct BaseUrl;

impl BaseUrl {
    pub fn get() -> String {
        BASE_URL.clone()
    }
}

pub static BASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("BASE_URL").expect("BASE_URL not found")
});