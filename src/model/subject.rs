#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct Subject {
    pub id: i64,
    pub subject_name: String,
    pub is_delete: i64,
    pub gmt_create: String,
    pub gmt_modified: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SubjectInfo {
    pub id: i64,
    pub subject_name: String,
}

