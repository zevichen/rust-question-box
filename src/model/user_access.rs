use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserAccess {
    pub id: i32,
    pub user_id: i32,
    pub access_id: i32,
    pub access_name: String,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub is_delete: i32,
}

