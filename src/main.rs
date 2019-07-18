mod data;
mod env;
mod shared;
mod time;

mod mem_store;
mod redis_store;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::env::{info_env, REDIS_URL};

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info_env();

    if REDIS_URL.is_none() {
        crate::mem_store::run_server()
    } else {
        crate::redis_store::run_server()
    }
}
