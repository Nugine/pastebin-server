mod handler;
mod state;
mod store;

pub use store::{LruValueSize, WithDeadTime};

use self::handler::{find_record, save_record};
use self::state::{State, Store, StoreLock};
use crate::env::{ADDR, CLEAN_DURATION, MAX_STORE_SIZE};
use crate::shared::resource::{json_post_config, FIND_RECORD_ROUTE, SAVE_RECORD_ROUTE};
use crate::time::{now_nano, NanoTime};

use std::thread;
use std::time::Duration;

use actix_web::{web, App, HttpServer};

fn gc(store: &mut Store, now: NanoTime) {
    let before_size = store.total_value_size();
    let before_count = store.item_count();
    let removed_count = {
        let cnt = store.clean(now);
        store.shrink();
        cnt
    };
    let stw_time = now_nano() - now;
    let after_size = store.total_value_size();
    let after_count = store.item_count();

    info!(
        "CLEAN stw: {} ns, removed: {}, store_size: {} -> {}, item_count: {} -> {}",
        stw_time, removed_count, before_size, after_size, before_count, after_count
    );
}

fn start_gc(store_lock: StoreLock) {
    thread::spawn(move || loop {
        // write store
        // assert: store_lock.write never returns Err or paincs
        let mut store = store_lock.write().unwrap();
        let now = now_nano();
        if store.needs_clean(now) {
            gc(&mut *store, now);
        }

        // release writer lock
        drop(store);

        thread::sleep(Duration::from_millis(*CLEAN_DURATION));
    });
}

pub fn run_server() -> std::io::Result<()> {
    let state = State::new(*MAX_STORE_SIZE);
    start_gc(state.store_lock.clone());

    info!("server start at {}", &*ADDR);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(web::resource(FIND_RECORD_ROUTE).route(web::get().to(find_record)))
            .service(
                web::resource(SAVE_RECORD_ROUTE)
                    .route(web::post().to(save_record))
                    .data(json_post_config()),
            )
    })
    .workers(1)
    .bind(&*ADDR)?
    .run()
}
