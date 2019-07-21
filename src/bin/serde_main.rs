
#[macro_use]
extern crate serde;
extern crate serde_json;

use serde_json::Value;

fn main() {
    let post = Post {
        id: Option::Some(1),
        title: "a",
        body: "body",
        user_id: 1,
    };

    let a = serde_json::to_string(&post).unwrap();
    println!("{}", a);

    let json =r#"{"id":1,"title":"a","body":"body","userId":1}"#;
    let x:Value = serde_json::from_str(json).unwrap();
    println!("{:?}",x);

}

#[derive(Debug, Serialize, Deserialize)]
struct Post<'a> {
    id: Option<i32>,
    title: &'a str,
    body: &'a str,
    #[serde(rename = "userId")]
    user_id: i32,
}