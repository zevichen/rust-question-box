use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Subject {
    pub id: i32,
    pub subject_name: String,
    pub is_delete: i32,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

