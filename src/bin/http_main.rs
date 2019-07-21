extern crate reqwest;

fn main() {
    let body = reqwest::get("https://www.rust-lang.org").expect("request error")
        .text().expect("text error.");

    println!("body = {:?}", body);
}
