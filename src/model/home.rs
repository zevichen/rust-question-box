#[derive(Clone, Debug, Serialize, Deserialize, PartialEq,Default)]
#[serde(default)]
pub struct HomeSubjectInfo {
    pub id: i64,
    pub subject_name: String,
    pub count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct HomeInfo {
    pub subject_infos: Vec<HomeSubjectInfo>
}