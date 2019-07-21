use actix::fut::err;
use actix_http::Error;
use actix_web::HttpResponse;
use futures::Future;
use futures::future::ok;
use serde_json::Value;
use serde_json::value::Value::Number;

use crate::model::content::ApiResponse;

/// 小程序获取session
pub fn code_session(code: &str) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut url = "https://api.weixin.qq.com/sns/jscode2session".to_string();
    url.push_str("?appid=wx38a0c021af15f58e");
    url.push_str("&secret=a0a862fd9d65ca85da7be58d4eee0eab");
    url.push_str(format!("&js_code={}", code).as_str());
    url.push_str("&grant_type=authorization_code");

    let response = reqwest::get(&url).map_err(|e| e).and_then(|r| Ok(r));
    if response.is_err() {
        return ok(HttpResponse::Ok().json(ApiResponse { message: response.err().unwrap().to_string().as_str(), data: "", ..Default::default() }));
    }


    let data: Value = response.unwrap().json().unwrap();
    println!("errcode = {:?}", data.get("errcode").unwrap());
    println!("{:?}", data);

    let errcode = data.get("errcode").unwrap().as_i64().unwrap();
    if errcode != 0 {
        let errmsg = data.get("errmsg").unwrap().as_str().unwrap();
        warn!("miniapp code2session errmsg={}", errmsg);
        return ok(HttpResponse::Ok().json(ApiResponse { message: errmsg, data: "", ..Default::default() }));
    }
    ok(HttpResponse::Ok().json(ApiResponse { data: "", ..Default::default() }))
}