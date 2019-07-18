use super::super::state::State;
use crate::data::dto::{ErrRes, FindRecordRes, SaveRecordReq};
use crate::data::key::key_to_nano;
use crate::env::REDIS_URL;
use crate::shared::error::HandlerError;
use crate::time::nano_to_sec;

use actix_web::{web, HttpRequest, HttpResponse, Result};

// path: /record/{key}
pub fn find_record(state: web::Data<State>, req: HttpRequest) -> Result<HttpResponse> {
    let key = req.match_info().get("key").unwrap();

    // (key: String) -> (nano: NanoTime)
    let nano = key_to_nano(&key).ok_or_else(|| HandlerError::bad_request(ErrRes::bad_key()))?;

    let mut store = state.store.borrow_mut();

    // access record
    let mut try_access = || -> Result<Option<(u64, String)>, HandlerError> {
        let log_error = |err| {
            error!("REDIS: {}", err);
        };
        let conv_error = |err| {
            log_error(err);
            HandlerError::internal_server_error(ErrRes::redis_error())
        };

        // first try
        if let Ok(o) = store.access(&key).map_err(log_error) {
            return Ok(o);
        };

        store
            .try_reopen(REDIS_URL.as_ref().unwrap())
            .map_err(conv_error)?;
        // second try
        store.access(&key).map_err(conv_error)
    };

    let (access_count, json_string) =
        try_access()?.ok_or_else(|| HandlerError::not_found(ErrRes::record_not_found()))?;

    // assert: redis json_string is valid
    let value: SaveRecordReq = serde_json::from_str(&json_string).unwrap();

    // construct response
    let resp = FindRecordRes {
        title: &value.title,
        lang: &value.lang,
        content: &value.content,
        saving_time: nano_to_sec(nano),
        expiration: value.expiration,
        view_count: access_count,
    };

    info!("FIND key = {}", key);
    Ok(HttpResponse::Ok().json(resp))
}
