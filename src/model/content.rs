use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Default)]
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
pub struct ApiRequest {
    pub token: String
}