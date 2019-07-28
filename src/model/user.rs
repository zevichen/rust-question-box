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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewUser<'a> {
    pub uuid: &'a str,
    pub nick_name: &'a str,
    pub union_id: &'a str,
    pub gmt_modified: &'a str,
    pub gmt_create: &'a str,
    pub source: &'a str,
    pub is_delete: i32,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub union_id: String,
    pub icon: String,
    pub nick_name: String,
    pub token: String,
    pub uuid: String,
}
