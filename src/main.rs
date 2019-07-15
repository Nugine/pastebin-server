mod data;
mod env;
mod handler;
mod store;
mod util;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::env::{info_env, ADDR, CLEAN_DURATION, INIT_CAPACITY, MAX_STORE_SIZE};
use crate::handler::{find_record_resource, save_record_resource, State, StoreLock};
use crate::store::time::now_nano;

use std::thread;
use std::time::Duration;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

fn start_gc(store_lock: StoreLock) {
    thread::spawn(move || loop {
        // read store
        // assert: store_lock.read never returns Err or paincs
        let store = store_lock.read().unwrap();
        let now = now_nano();
        let needs_clean = store.needs_clean(now);

        // release reader lock
        drop(store);

        if needs_clean {
            // write store
            // assert: store_lock.write never returns Err or paincs
            let mut store = store_lock.write().unwrap();

            info!("CLEAN start: store_size = {}", store.total_value_size());
            store.clean(now);
            info!("CLEAN end: store_size = {}", store.total_value_size());

            // release writer lock
            drop(store);
        }
        thread::sleep(Duration::from_millis(*CLEAN_DURATION));
    });
}

fn run_server(state: State) -> std::io::Result<()> {
    info!("server start at {}", &*ADDR);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(find_record_resource())
            .service(save_record_resource())
    })
    .workers(1)
    .bind(&*ADDR)?
    .run()
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info_env();

    let state = State::new(*MAX_STORE_SIZE, *INIT_CAPACITY);
    start_gc(state.store_lock.clone());
    run_server(state)
}
