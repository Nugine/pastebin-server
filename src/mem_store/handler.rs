mod find_record;
mod save_record;

use self::find_record::find_record;
use self::save_record::save_record;

use crate::data::dto::{ErrRes, SaveRecordReq};
use crate::env::MAX_POST_SIZE;
use crate::shared::error::HandlerError;

use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, Resource, ResponseError};

pub fn find_record_resource() -> Resource {
    web::resource("/record/{key}").route(web::get().to(find_record))
}

pub fn save_record_resource() -> Resource {
    web::resource("/record")
        .route(web::post().to(save_record))
        .data(web::Json::<SaveRecordReq>::configure(|cfg| {
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
        }))
}
