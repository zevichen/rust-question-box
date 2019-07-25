use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Title {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub question_image: String,
    pub question_desc: String,
    pub answer_image: String,
    pub answer_desc: String,
    pub degree: String,
    pub title_type: String,
    pub subject_id: i32,
    pub subject_name: String,
    pub tags: String,
    pub is_delete: i32,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq,Default)]
#[serde(default)]
pub struct QuestionForm {
    pub token: String,
    pub question_desc: String,
    pub answer_desc: String,
    pub degree: String,
    pub title_type: String,
    pub subject_id: i32,
    pub subject_name: String,
    pub tags: String,
}

