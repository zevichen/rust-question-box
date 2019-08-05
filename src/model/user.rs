use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub real_name: String,
    pub user_name: String,
    pub nick_name: String,
    pub mobile: String,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub icon: String,
    pub password: String,
    pub salt: String,
    pub union_id: String,
    pub source: String,
    pub is_delete: i64,
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UuidSigninUser {
    pub nick_name: String,
    pub union_id: String,
    pub session_key: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub uuid:String,
    pub user_name:String,
    pub nick_name:String,
    pub icon:String,
}
