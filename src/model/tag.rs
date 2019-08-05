#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
    pub uuid: String,
    pub gmt_create: String,
    pub gmt_modified: String,
    pub is_delete: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagForm {
    pub token: String
}


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagInfo {
    pub id: i64,
    pub tag_name: String,
}



