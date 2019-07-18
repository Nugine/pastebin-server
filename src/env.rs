//! MAX_STORE_SIZE: 100 MB
//!
//! MAX_POST_SIZE: 32 KB
//!
//! MAX_EXPIRATION: 7 days
//!
//! CLEAN_DURATION: 5000 ms
//!
//! ADDR: "localhost:8088"
//!
//! CRYPT_KEY: "magic"
//!
//! REDIS_URL: None

use crate::time::SecTime;

use std::env;
use std::str::FromStr;

fn parse<T: FromStr>(var: &'static str, default: T) -> T {
    env::var(var)
        .ok()
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(default)
}

const DEFAULT_ADDR: &'static str = "localhost:8088";
const DEFAULT_CRYPT_KEY: &'static str = "magic";

lazy_static! {
    pub static ref MAX_STORE_SIZE: usize = { parse("PASTEBIN_MAX_STORE_SIZE", 100 * 1024 * 1024) };
    pub static ref MAX_POST_SIZE: usize = { parse("PASTEBIN_MAX_POST_SIZE", 32 * 1024) };
    pub static ref MAX_EXPIRATION: SecTime = { parse("PASTEBIN_MAX_EXPIRATION", 7 * 24 * 60 * 60) };
    pub static ref CLEAN_DURATION: u64 = { parse("PASTEBIN_CLEAN_DURATION", 5000) };
    pub static ref ADDR: String = { env::var("PASTEBIN_ADDR").unwrap_or(DEFAULT_ADDR.into()) };
    pub static ref CRYPT_KEY: String =
        { env::var("PASTEBIN_CRYPT_KEY").unwrap_or(DEFAULT_CRYPT_KEY.into()) };
    pub static ref REDIS_URL: Option<String> = { env::var("REDIS_URL").ok() };
}

pub fn info_env() {
    info!("ADDR: {}", *ADDR);
    info!("MAX_POST_SIZE: {} bytes", *MAX_POST_SIZE);
    // info!("CRYPT_KEY: {}", *CRYPT_KEY);
    match *REDIS_URL {
        Some(ref redis_url) => {
            info!("REDIS_URL: {}", redis_url);
        }
        None => {
            info!("MAX_STORE_SIZE: {} bytes", *MAX_STORE_SIZE);
            info!("MAX_EXPIRATION: {} s", *MAX_EXPIRATION);
            info!("CLEAN_DURATION: {} ms", *CLEAN_DURATION);
        }
    }
}
