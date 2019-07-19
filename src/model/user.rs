use chrono::NaiveDateTime;
use rusqlite::ToSql;
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
    pub session_key:String,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserInfo {
    pub id: u32,
    pub is_login: bool,
    pub uuid: String,
    pub union_id: String,
    pub icon: String,
    pub nick_name: String,
    pub token:String,
}

impl UserInfo {
    pub fn new() -> Self {
        UserInfo {
            id: 0,
            is_login: false,
            uuid: "",
            union_id: "",
            icon: "",
            nick_name: "",
            token: "",
        }
    }
}

