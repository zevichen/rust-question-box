use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiResponse<'a, T> {
    pub code: u32,
    pub message: &'a str,
    pub data: T,
    pub page_num: u32,
    pub page_size: u32,
    pub page_total: u32,
}