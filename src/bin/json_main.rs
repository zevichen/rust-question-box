#[macro_use]
extern crate serde;
extern crate serde_json;

fn main() {
    let json = serde_json::to_string(&Person {
        id: 1,
        name: "zhangsan".to_string(),
        gmt_create: "2019-10-10".to_string(),
    }).unwrap();
    println!("{}", json);

    println!("--------------------");

    let x = serde_json::from_str::<Person>(&json);
    println!("{:?}", x);
}


#[derive(Clone, Deserialize, Serialize, Debug)]
struct Person {
    id: i32,
    name: String,
    gmt_create: String,
}