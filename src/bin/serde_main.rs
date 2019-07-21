extern crate serde;
extern crate serde_json;
fn main() {




}
#[derive(Debug, Serialize, Deserialize)]
struct Post<'a> {
    id: Option<i32>,
    title: &'a str,
    body: &'a str,
    #[serde(rename = "userId")]
    user_id: i32,
}