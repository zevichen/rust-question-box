use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
    pub user_id: String,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub is_delete: i32,
}

