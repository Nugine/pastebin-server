mod env;
mod store;
mod util;
mod data;
mod handler;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use dotenv::dotenv;

use crate::env::info_env;

fn main() {
    dotenv().ok();
    env_logger::init();
    info_env();
    println!("Hello, world!");
}
