# question-box

```rust
/// login
//pub fn login(item: web::Json<UuidSigninUser>,
//             pool: web::Data<SqlitePool>,
//             session: Session) -> impl Future<Item=HttpResponse, Error=Error> {
//    if log_enabled!(Level::Debug) {
//        info!("Params = {:?}", &item)
//    }
//
//    let opt_user_info: Option<UserInfo> = session.get("user_info").unwrap_or_default();
//    if !opt_user_info.is_none() {
//        return ok(HttpResponse::Ok().json(ApiResponse { data: opt_user_info.unwrap().uuid, ..Default::default() }));
//    }
//
//    if item.union_id.is_empty() || item.session_key.is_empty() {
//        return ok(HttpResponse::Ok().json(ApiResponse { message: "union_id or session_key is empty.".to_owned(),data:"", ..Default::default() }));
//    }
//
//    let conn = pool.get().unwrap();
//
//    let mut uuid = conn.query_row("select uuid from user where union_id = $1 limit 1", &[&item.union_id], |row| {
//        row.get::<_, String>(0)
//    }).unwrap();
//
//    if !uuid.is_empty() {
//        return ok(HttpResponse::Ok().json(ApiResponse { data: uuid, ..Default::default() }));
//    }
//
//    let now = Local::now().format(common::DATE_FORMAT_1).to_string();
//    let mut rng = rand::thread_rng();
//    uuid = format!("{}", rng.next_u64());
//
//    conn.execute(
//        "insert or replace into user (uuid,nick_name,union_id,gmt_create,gmt_modified,source,is_delete) \
//            values ($1,$2,$3,$4,$5,$6,0)",
//        &[&uuid, &item.nick_name, &item.union_id, &now, &now, common::SOURCE_WECHAT],
//    ).expect("user insert or replace failed.");
//
//    let user_info = UserInfo { is_login: true, uuid:uuid.to_owned(), nick_name: item.nick_name.to_owned(), ..Default::default() };
//    session.set::<UserInfo>("user_info", user_info).expect("Failed setting user_info into session");
//
//    return ok(HttpResponse::Ok().json(ApiResponse { data: uuid, ..Default::default() }));
//}
```


