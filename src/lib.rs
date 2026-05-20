use worker::*;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

const DATA_JSON: &str = include_str!("../data.json");
const ACCESS_PASSWORD: &str = "dabendi66";
const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "Admin@123";
const JWT_SECRET: &str = "apps-cf-jwt-secret-2026";

#[derive(Serialize, Deserialize, Clone)]
struct Category { id: i64, name: String, icon: String, sort_order: i64 }

#[derive(Serialize, Deserialize, Clone)]
struct VersionItem { version: String, url: String }

#[derive(Serialize, Deserialize, Clone)]
struct Tool {
    id: i64, name: String, description: String, url: String, icon: String,
    category_id: i64, tags: Vec<String>, platforms: Vec<String>,
    versions: Vec<VersionItem>, sort_order: i64, is_featured: bool,
}

#[derive(Serialize, Deserialize)]
struct AppData { categories: Vec<Category>, tools: Vec<Tool> }

#[derive(Serialize)]
struct ApiResponse { code: i64, message: String, data: serde_json::Value }

impl ApiResponse {
    fn ok(data: serde_json::Value) -> Self { Self { code: 0, message: "ok".into(), data } }
    fn err(code: i64, msg: &str) -> Self { Self { code, message: msg.into(), data: serde_json::Value::Null } }
}

#[derive(Deserialize)]
struct PasswordVerify { password: String }

#[derive(Deserialize)]
struct LoginRequest { username: String, password: String }

fn get_data() -> AppData { serde_json::from_str(DATA_JSON).unwrap() }

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn make_jwt(payload: &str) -> String {
    let header = b64e(r#"{"alg":"HS256","typ":"JWT"}"#);
    let body = b64e(payload);
    let sig = sha256_hex(&format!("{}.{}", header, body, ));
    format!("{}.{}.{}", header, body, &sig[..32])
}

fn b64e(input: &str) -> String {
    const C: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let b = input.as_bytes();
    let mut o = String::new();
    for ch in b.chunks(3) {
        let a = ch[0] as u32;
        let d = if ch.len()>1{ch[1] as u32}else{0};
        let e = if ch.len()>2{ch[2] as u32}else{0};
        let n = (a<<16)|(d<<8)|e;
        o.push(C[((n>>18)&0x3F)as usize]as char);
        o.push(C[((n>>12)&0x3F)as usize]as char);
        if ch.len()>1{o.push(C[((n>>6)&0x3F)as usize]as char);}
        if ch.len()>2{o.push(C[(n&0x3F)as usize]as char);}
    }
    o
}

fn json_resp(data: &ApiResponse) -> Result<Response> {
    let mut r = Response::from_json(data)?;
    r.headers_mut().set("Content-Type", "application/json")?;
    r.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    Ok(r)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        let mut r = Response::empty()?;
        r.headers_mut().set("Access-Control-Allow-Origin", "*")?;
        r.headers_mut().set("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS")?;
        r.headers_mut().set("Access-Control-Allow-Headers", "Content-Type,Authorization")?;
        r.headers_mut().set("Access-Control-Max-Age", "86400")?;
        return Ok(r);
    }
    Router::new()
        .get("/health", |_req, _env| {
            json_resp(&ApiResponse::ok(serde_json::json!({"status":"ok","version":"0.1.0"})))
        })
        .get("/api/categories", |_req, _env| {
            let data = get_data();
            json_resp(&ApiResponse::ok(serde_json::from_str(&serde_json::to_string(&data.categories)?)?))
        })
        .get("/api/tools", |req, _env| {
            let data = get_data();
            let url = req.url()?;
            let params: Vec<(String,String)> = url.query_pairs().map(|(k,v)|(k.to_string(),v.to_string())).collect();
            let mut tools = data.tools.clone();
            let page: usize = params.iter().find(|(k,_)|k=="page").and_then(|(_,v)|v.parse().ok()).unwrap_or(1);
            let ps: usize = params.iter().find(|(k,_)|k=="page_size").and_then(|(_,v)|v.parse().ok()).unwrap_or(100);
            if let Some(cid) = params.iter().find(|(k,_)|k=="category_id").and_then(|(_,v)|v.parse::<i64>().ok()) {
                if cid>0 {tools.retain(|t|t.category_id==cid);}
            }
            if let Some(s) = params.iter().find(|(k,_)|k=="search").map(|(_,v)|v.to_lowercase()) {
                if !s.is_empty() {tools.retain(|t|t.name.to_lowercase().contains(&s)||t.description.to_lowercase().contains(&s));}
            }
            let total = tools.len();
            let items: Vec<_> = tools.into_iter().skip((page-1)*ps).take(ps).collect();
            json_resp(&ApiResponse::ok(serde_json::json!({"items":items,"total":total,"page":page,"page_size":ps})))
        })
        .post_async("/api/settings/access-password/verify", |mut req, _env| async move {
            let body: PasswordVerify = req.json().await?;
            if body.password == ACCESS_PASSWORD {
                json_resp(&ApiResponse::ok(serde_json::json!({"verified":true})))
            } else {
                json_resp(&ApiResponse::err(1002, "Wrong password"))
            }
        })
        .post_async("/api/auth/login", |mut req, _env| async move {
            let body: LoginRequest = req.json().await?;
            if body.username == ADMIN_USER && body.password == ADMIN_PASSWORD {
                let token = make_jwt(&format!(r#"{{"username":"{}","role":"admin"}}"#, ADMIN_USER));
                json_resp(&ApiResponse::ok(serde_json::json!({"token":token,"username":ADMIN_USER,"role":"admin"})))
            } else {
                json_resp(&ApiResponse::err(1002, "Invalid credentials"))
            }
        })
        .get("/api/stats", |_req, _env| {
            let data = get_data();
            json_resp(&ApiResponse::ok(serde_json::json!({"total_tools":data.tools.len(),"total_categories":data.categories.len()})))
        })
        .run(req, env)
        .await
}
