use super::state::State;
use crate::data::dto::{ErrRes, SaveRecordReq, SaveRecordRes};
use crate::data::key::nano_to_key;
use crate::data::record::Record;
use crate::env::MAX_EXPIRATION;
use crate::store::time::{nano_to_sec, now_nano, sec_to_nano};

use actix_web::{web, HttpResponse};

// path: /record
pub fn save_record(state: web::Data<State>, dto: web::Json<SaveRecordReq>) -> HttpResponse {
    if dto.expiration > *MAX_EXPIRATION {
        return HttpResponse::BadRequest().json(ErrRes::too_long_expiration());
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
    };

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = state.store_lock.write().unwrap();
    store.save(now, record, dead_time);

    let store_size = store.total_value_size();
    let key = nano_to_key(now);

    info!("SAVE key = {}, store_size = {}", key, store_size);

    HttpResponse::Ok().json(SaveRecordRes { key: &key })
}
