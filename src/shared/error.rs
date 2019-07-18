use crate::data::dto::ErrRes;

use std::fmt::Display;

use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug)]
pub struct HandlerError<'a> {
    pub err_res: ErrRes<'a>,
    pub status_code: StatusCode,
}

impl<'a> Display for HandlerError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.err_res).unwrap())
    }
}

impl<'a> ResponseError for HandlerError<'a> {
    fn render_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json2(&self.err_res)
    }
}

impl<'a> HandlerError<'a> {
    pub fn bad_request(err_res: ErrRes<'a>) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            err_res,
        }
    }

    pub fn not_found(err_res: ErrRes<'a>) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            err_res,
        }
    }

    pub fn internal_server_error(err_res: ErrRes<'a>) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            err_res,
        }
    }
}
