/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
/// iss: jwt签发者
/// sub: jwt所面向的用户
/// aud: 接收jwt的一方
/// exp: jwt的过期时间，这个过期时间必须要大于签发时间
/// nbf: 定义在什么时间之前，该jwt都是不可用的.
/// iat: jwt的签发时间
/// jti: jwt的唯一身份标识，主要用来作为一次性token,从而回避重放攻击。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    // unionid
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    // #[validate(length(min = "1", max = "1000000"))]
    pub union_id: String,
    pub session_key: String,
    pub nick_name: String,
}