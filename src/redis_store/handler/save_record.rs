use super::super::state::State;
use crate::data::dto::{ErrRes, SaveRecordReq, SaveRecordRes};
use crate::data::key::nano_to_key;
use crate::env::MAX_EXPIRATION;
use crate::env::REDIS_URL;
use crate::shared::error::HandlerError;
use crate::time::now_nano;

use actix_web::{web, HttpResponse, Result};

// path: /record
pub fn save_record(state: web::Data<State>, dto: web::Json<SaveRecordReq>) -> Result<HttpResponse> {
    if dto.expiration > *MAX_EXPIRATION {
        return Err(HandlerError::bad_request(ErrRes::too_long_expiration()).into());
    }

    let mut store = state.store.borrow_mut();

    let now = now_nano();
    let key = nano_to_key(now);
    // assert: SaveRecordReq is valid
    let json_string = serde_json::to_string(&dto.0).unwrap();

    let mut try_save = || -> Result<(), HandlerError> {
        let log_error = |err| {
            error!("REDIS: {}", err);
        };
        let conv_error = |err| {
            log_error(err);
            HandlerError::internal_server_error(ErrRes::redis_error())
        };

        // first try
        if store
            .save(&key, &json_string, dto.expiration)
            .map_err(log_error)
            .is_ok()
        {
            return Ok(());
        }

        store
            .try_reopen(REDIS_URL.as_ref().unwrap())
            .map_err(conv_error)?;

        // second try
        store
            .save(&key, &json_string, dto.expiration)
            .map_err(conv_error)
    };

    try_save()?;

    info!("SAVE key = {}", key);

    Ok(HttpResponse::Ok().json(SaveRecordRes { key: &key }))
}
