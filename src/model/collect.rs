

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CollectForm {
    pub page_index: i32,
    pub page_size: i32,
    pub token: String,
    pub question_id:i32,
}