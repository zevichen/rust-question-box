extern crate reqwest;

use serde_json::Value;

fn main() {
    let code = "null";

    let mut url = "https://api.weixin.qq.com/sns/jscode2session".to_string();
    let js_code = "&js_code=".to_owned() + code;

    url.push_str("?appid=wx38a0c021af15f58e");
    url.push_str("&secret=a0a862fd9d65ca85da7be58d4eee0eab");
    url.push_str(&js_code);
    url.push_str("&grant_type=authorization_code");

    let response = reqwest::get(&url).map_err(|e| e).and_then(|r| Ok(r));
    if response.is_ok(){
        let result:Value = response.unwrap().json().unwrap();
        println!("errcode = {:?}",result.get("errcode").unwrap());
        println!("{:?}", result);
    }else{
        println!("{}",response.unwrap_err())
    }
}