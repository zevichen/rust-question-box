use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub vip_level: i32,
    pub gmt_vip: NaiveDateTime,
    pub is_delete: i32,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

