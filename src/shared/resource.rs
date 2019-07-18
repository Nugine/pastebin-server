use crate::data::dto::{ErrRes, SaveRecordReq};
use crate::env::MAX_POST_SIZE;
use crate::shared::error::HandlerError;

use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, ResponseError};

pub const FIND_RECORD_ROUTE: &'static str = "/record/{key}";
pub const SAVE_RECORD_ROUTE: &'static str = "/record";

pub fn json_post_config() -> <web::Json<SaveRecordReq> as FromRequest>::Config {
    web::Json::<SaveRecordReq>::configure(|cfg| {
        cfg.error_handler(|err, _| {
            actix_web::error::InternalError::from_response(
                err,
                HandlerError {
                    status_code: StatusCode::BAD_REQUEST,
                    err_res: ErrRes::too_long_content(),
                }
                .render_response(),
            )
            .into()
        })
        .limit(*MAX_POST_SIZE)
    })
}
