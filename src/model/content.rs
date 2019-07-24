use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiResponse {
    pub code: u32,
    pub message: String,
    pub data: String,
    pub page_num: u32,
    pub page_size: u32,
    pub page_total: u32,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiRequest {
    pub token: String,
    pub code:String
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppState{
    pub secret:String,
}