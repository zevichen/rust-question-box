use serde::Deserialize;

use share::common::EMPTY_STRING;

#[derive(Deserialize, Serialize, Debug)]
pub struct Content<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
    pub page_num: u32,
    pub page_size: u32,
    pub page_total: u32,
}


impl<T> Content<T>
{
    pub fn new(t: T) -> Self {
        Content {
            code: 0,
            message: EMPTY_STRING.to_owned(),
            data: t,
            page_num: 1,
            page_size: 10,
            page_total: 0,
        }
    }

    pub fn err(t: T, msg: &str) -> Self {
        Content {
            code: 100,
            message: msg.to_owned(),
            data: t,
            page_num: 1,
            page_size: 10,
            page_total: 0,
        }
    }

    pub fn err_code(code: u32, msg: String, t: T) -> Self {
        Content {
            code,
            message: msg,
            data: t,
            page_num: 1,
            page_size: 10,
            page_total: 0,
        }
    }
}
