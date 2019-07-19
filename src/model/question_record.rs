use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TitleRecord {
    pub id: i32,
    pub question_id: i32,
    pub status: String,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub remark: String,
}

