use super::state::State;
use crate::data::dto::{ErrRes, FindRecordRes};
use crate::data::key::key_to_nano;
use crate::or_return;

use actix_web::{web, HttpRequest, HttpResponse};

// path: /record/:key
pub fn find_record(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
    let key = req.match_info().get("key").unwrap();
    // (key: String) -> (nano: NanoTime)
    let nano = or_return!(
        key_to_nano(&key),
        HttpResponse::BadRequest().json(ErrRes::bad_key())
    );

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = state.store_lock.write().unwrap();
    let store_size = store.total_value_size();

    // access record
    let item = or_return!(
        store.access(nano),
        HttpResponse::NotFound().json(ErrRes::record_not_found())
    );

    // construct response
    let resp = FindRecordRes {
        title: &item.value.title,
        lang: &item.value.lang,
        content: &item.value.content,
        saving_time: item.value.saving_time,
        expiration: item.value.expiration,
        view_count: item.access_count,
    };

    info!("FIND key = {}, store_size = {}", key, store_size);
    HttpResponse::Ok().json(resp)
}
