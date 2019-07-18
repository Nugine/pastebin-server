mod data;
mod env;
mod shared;
mod time;

mod mem_store;
// mod redis_store;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::env::info_env;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info_env();

    crate::mem_store::run_server()
}
