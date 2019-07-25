use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
    pub page_num: u32,
    pub page_size: u32,
    pub page_total: u32,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct ApiRequest {
    pub token: String,
    pub code: String,
}


impl<T> ApiResponse<T> {
    pub fn default(data: T) -> Self {
        ApiResponse {
            code: 0,
            message: "".to_owned(),
            data,
            page_num: 0,
            page_size: 0,
            page_total: 0,
            token: "".to_owned(),
        }
    }

    pub fn fail(msg: String, data: T) -> Self {
        ApiResponse {
            code: 1,
            message: msg,
            data,
            page_num: 0,
            page_size: 0,
            page_total: 0,
            token: "".to_owned(),
        }
    }

    pub fn fail_code(code: u32, msg: String, data: T) -> Self {
        ApiResponse {
            code,
            message: msg,
            data,
            page_num: 0,
            page_size: 0,
            page_total: 0,
            token: "".to_string(),
        }
    }

    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 0,
            message: "成功".to_owned(),
            data,
            page_num: 0,
            page_size: 0,
            page_total: 0,
            token: "".to_string(),
        }
    }
}