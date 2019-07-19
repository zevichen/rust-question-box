use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Behavior {
    pub id: i32,
    pub user_id: String,
    pub bhv_type: String,
    pub reference: String,
    pub ip: String,
    pub is_login: String,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

