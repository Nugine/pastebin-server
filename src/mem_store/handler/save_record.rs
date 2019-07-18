use super::super::state::State;
use crate::data::dto::{ErrRes, SaveRecordReq, SaveRecordRes};
use crate::data::key::nano_to_key;
use crate::data::record::Record;
use crate::env::MAX_EXPIRATION;
use crate::shared::error::HandlerError;
use crate::time::{nano_to_sec, now_nano, sec_to_nano};

use actix_web::{web, HttpResponse, Result};

// path: /record
pub fn save_record(state: web::Data<State>, dto: web::Json<SaveRecordReq>) -> Result<HttpResponse> {
    if dto.expiration > *MAX_EXPIRATION {
        return Err(HandlerError::bad_request(ErrRes::too_long_expiration()).into());
    }

    let now = now_nano();
    let saving_time = nano_to_sec(now);
    let dead_time = now + sec_to_nano(dto.expiration); // assert: now.add(expiraton) never overflows

    let record = Record {
        title: dto.0.title,
        lang: dto.0.lang,
        content: dto.0.content,
        saving_time,
        expiration: dto.0.expiration,
        dead_time,
    };

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = state.store_lock.write().unwrap();
    store.save(now, record);

    let store_size = store.total_value_size();
    let item_count = store.item_count();
    let key = nano_to_key(now);

    info!(
        "SAVE key = {}, store_size = {}, item_count = {}",
        key, store_size, item_count
    );

    Ok(HttpResponse::Ok().json(SaveRecordRes { key: &key }))
}
