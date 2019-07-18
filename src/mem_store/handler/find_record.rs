use super::super::state::State;
use crate::data::dto::{ErrRes, FindRecordRes};
use crate::data::key::key_to_nano;
use crate::shared::error::HandlerError;

use actix_web::{web, HttpRequest, HttpResponse, Result};

// path: /record/{key}
pub fn find_record(state: web::Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let key = req.match_info().get("key").unwrap();

    // (key: String) -> (nano: NanoTime)
    let nano = key_to_nano(&key).ok_or_else(|| HandlerError::bad_request(ErrRes::bad_key()))?;

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = state.store_lock.write().unwrap();

    // access record
    let item = store
        .access(nano)
        .ok_or_else(|| HandlerError::not_found(ErrRes::record_not_found()))?;

    // construct response
    let resp = FindRecordRes {
        title: &item.value.title,
        lang: &item.value.lang,
        content: &item.value.content,
        saving_time: item.value.saving_time,
        expiration: item.value.expiration,
        view_count: item.access_count,
    };

    info!("FIND key = {}", key);
    Ok(HttpResponse::Ok().json(resp))
}
