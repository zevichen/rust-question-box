use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Exercise {
    pub id: i32,
    pub tag_name: String,
    pub subject_id: i32,
    pub subject_name: String,
    pub title: String,
    pub exercise_type: String,
    pub flag: String,
    pub is_delete: i32,
    pub title_id: i32,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

