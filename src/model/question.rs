use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub question_image: String,
    pub question_desc: String,
    pub answer_image: String,
    pub answer_desc: String,
    pub degree: String,
    pub question_type: String,
    pub subject_id: i64,
    pub subject_name: String,
    pub tags: String,
    pub is_delete: i64,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct QuestionForm {
    pub question_id: i64,
    pub token: String,
    pub name: String,
    pub question_image: String,
    pub question_desc: String,
    pub answer_image: String,
    pub answer_desc: String,
    pub degree: String,
    pub question_type: String,
    pub subject_id: String,
    pub subject_name: String,
    pub tags: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct QuestionInfo {
    pub id: i64,
    pub name: String,
    pub question_image: String,
    pub question_desc: String,
    pub answer_image: String,
    pub answer_desc: String,
    pub degree: String,
    pub question_type: String,
    pub subject_id: i64,
    pub subject_name: String,
    pub tags: String,
    pub gmt_create: String,
}



