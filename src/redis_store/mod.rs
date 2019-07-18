mod handler;
mod state;
mod store;

use self::handler::{find_record, save_record};
use self::state::State;
use crate::env::{ADDR, REDIS_URL};
use crate::shared::resource::{json_post_config, FIND_RECORD_ROUTE, SAVE_RECORD_ROUTE};

use actix_web::{web, App, HttpServer};

pub fn run_server() -> std::io::Result<()> {
    info!("server start at {}", &*ADDR);

    HttpServer::new(move || {
        App::new()
            .data(State::new(REDIS_URL.as_ref().unwrap()))
            .service(web::resource(FIND_RECORD_ROUTE).route(web::get().to(find_record)))
            .service(
                web::resource(SAVE_RECORD_ROUTE)
                    .route(web::post().to(save_record))
                    .data(json_post_config()),
            )
    })
    // .workers(1)
    .bind(&*ADDR)?
    .run()
}
