use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
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
    pub is_delete: i32,
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UuidSigninUser {
    pub nick_name: String,
    pub union_id: String,
    pub session_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewUser {
    pub uuid: String,
    pub nick_name: String,
    pub union_id: String,
    pub gmt_modified: String,
    pub gmt_create: String,
    pub source: String,
    pub is_delete: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
    pub id: u32,
    pub is_login: bool,
    pub uuid: String,
    pub union_id: String,
    pub icon: String,
    pub nick_name: String,
    pub token: String,
}

impl UserInfo {
    pub fn new() -> Self {
        UserInfo {
            id: 0,
            is_login: false,
            uuid: "".to_owned(),
            union_id: "".to_owned(),
            icon: "".to_owned(),
            nick_name: "".to_owned(),
            token: "".to_owned(),
        }
    }
}

