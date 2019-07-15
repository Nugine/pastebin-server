use crate::store::time::SecTime;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SaveRecordReq {
    pub title: String,
    pub lang: String,
    pub content: String,
    pub expiration: SecTime,
}

#[derive(Serialize)]
pub struct SaveRecordRes<'a> {
    pub key: &'a str,
}

#[derive(Serialize)]
pub struct FindRecordRes<'a> {
    pub title: &'a str,
    pub lang: &'a str,
    pub content: &'a str,
    pub saving_time: SecTime,
    pub expiration: SecTime,
    pub view_count: u64,
}

#[derive(Serialize, Debug)]
pub struct ErrRes<'a> {
    pub code: i32,
    pub message: &'a str,
}

macro_rules! define_err_res {
    ($method:ident,$code:expr,$msg:expr) => {
        impl<'a> ErrRes<'a> {
            #[inline]
            pub fn $method() -> Self {
                ErrRes { code: $code,message: $msg,}
            }
        }
    };
}

define_err_res!(bad_key,1001,"Can not parse key");
define_err_res!(record_not_found,1002,"Can not find record");
define_err_res!(too_long_expiration,1003,"Too long expiration");
define_err_res!(too_long_content,1004,"Too long content");

#[cfg(test)]
#[test]
fn test_err_res() {
    let p = |err:ErrRes|{println!("{}",serde_json::to_string(&err).unwrap())};
    p(ErrRes::bad_key());
    p(ErrRes::record_not_found());
    p(ErrRes::too_long_expiration());
    p(ErrRes::too_long_content());
}