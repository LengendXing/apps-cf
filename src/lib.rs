use worker::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DATA_JSON: &str = include_str!("../data.json");
const INDEX_HTML: &str = include_str!("../frontend/dist/index.html");
const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "Admin@123";

// === Data Structures ===

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

#[derive(Serialize, Deserialize, Clone)]
struct User { id: i64, username: String, email: String, password: String, role: String, is_active: bool }

#[derive(Serialize, Deserialize, Clone)]
struct AuditLog { id: i64, user_id: i64, action: String, target_type: String, target_id: Option<i64>, details: String, created_at: String }

#[derive(Serialize, Deserialize, Clone)]
struct Script {
    id: i64, tool_id: i64, name: String, content: String,
    platform: String, tags: Vec<String>, sort_order: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    id: i64, name: String, format: String,
    content: String, sort_order: i64, copy_count: i64,
}

#[derive(Serialize, Deserialize)]
struct AppData { categories: Vec<Category>, tools: Vec<Tool> }

#[derive(Serialize)]
struct ApiResponse {
    code: i64, message: String,
    #[serde(skip_serializing_if = "Option::is_none")] detail: Option<String>,
    data: serde_json::Value,
}
impl ApiResponse {
    fn ok(data: serde_json::Value) -> Self { Self { code: 0, message: "ok".into(), detail: None, data } }
    fn err(code: i64, msg: &str) -> Self { Self { code, message: msg.into(), detail: Some(msg.into()), data: serde_json::Value::Null } }
}

#[derive(Deserialize)] struct LoginRequest { username: String, password: String }
#[derive(Deserialize)] struct PasswordVerify { password: String }
#[derive(Deserialize)] struct PasswordUpdate { password: String }
#[derive(Deserialize)] struct CategoryInput { name: Option<String>, icon: Option<String>, sort_order: Option<i64> }
#[derive(Deserialize)] struct ToolInput {
    name: Option<String>, description: Option<String>, url: Option<String>, icon: Option<String>,
    category_id: Option<i64>, tags: Option<Vec<String>>, platforms: Option<Vec<String>>,
    versions: Option<Vec<VersionItem>>, sort_order: Option<i64>, is_featured: Option<bool>,
}
#[derive(Deserialize)] struct UserInput { is_active: Option<bool>, role: Option<String> }
#[derive(Deserialize)] struct ScriptInput {
    tool_id: Option<i64>, name: Option<String>, content: Option<String>,
    platform: Option<String>, tags: Option<Vec<String>>, sort_order: Option<i64>,
}
#[derive(Deserialize)] struct ConfigInput {
    name: Option<String>, format: Option<String>,
    content: Option<String>, sort_order: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone)]
struct SystemConfig { layout: String }
#[derive(Deserialize)] struct SystemConfigInput { layout: Option<String> }

// === KV Helpers ===

async fn ensure_seeded(kv: &KvStore) -> Result<()> {
    if kv.get("seeded").text().await?.is_some() { return Ok(()); }
    let data: AppData = serde_json::from_str(DATA_JSON).unwrap();
    kv.put("categories", &data.categories)?.execute().await?;
    kv.put("tools", &data.tools)?.execute().await?;
    kv.put("users", &vec![User {
        id: 1, username: "admin".to_string(), email: "admin@apps-cf.dev".to_string(),
        password: ADMIN_PASSWORD.to_string(), role: "admin".to_string(), is_active: true,
    }])?.execute().await?;
    kv.put("audit_logs", &Vec::<AuditLog>::new())?.execute().await?;
    kv.put("scripts", &Vec::<Script>::new())?.execute().await?;
    kv.put("configs", &Vec::<Config>::new())?.execute().await?;
    kv.put("system_config", &SystemConfig { layout: "left".to_string() })?.execute().await?;
    kv.put("next_id", 1000_i64)?.execute().await?;
    kv.put("access_password", "dabendi66")?.execute().await?;
    kv.put("seeded", "1")?.execute().await?;
    Ok(())
}

async fn kv_categories(kv: &KvStore) -> Result<Vec<Category>> {
    Ok(kv.get("categories").json::<Vec<Category>>().await?.unwrap_or_default())
}

async fn kv_tools(kv: &KvStore) -> Result<Vec<Tool>> {
    Ok(kv.get("tools").json::<Vec<Tool>>().await?.unwrap_or_default())
}

async fn kv_users(kv: &KvStore) -> Result<Vec<User>> {
    Ok(kv.get("users").json::<Vec<User>>().await?.unwrap_or_default())
}

async fn kv_audit_logs(kv: &KvStore) -> Result<Vec<AuditLog>> {
    Ok(kv.get("audit_logs").json::<Vec<AuditLog>>().await?.unwrap_or_default())
}

async fn kv_scripts(kv: &KvStore) -> Result<Vec<Script>> {
    Ok(kv.get("scripts").json::<Vec<Script>>().await?.unwrap_or_default())
}

async fn kv_configs(kv: &KvStore) -> Result<Vec<Config>> {
    Ok(kv.get("configs").json::<Vec<Config>>().await?.unwrap_or_default())
}

async fn alloc_id(kv: &KvStore) -> Result<i64> {
    let cur: i64 = kv.get("next_id").json().await?.unwrap_or(1000);
    let next = cur + 1;
    kv.put("next_id", next)?.execute().await?;
    Ok(next)
}

async fn log_action(kv: &KvStore, user_id: i64, action: &str, target_type: &str, target_id: Option<i64>, details: &str) -> Result<()> {
    let mut logs = kv_audit_logs(kv).await?;
    let id = alloc_id(kv).await?;
    logs.push(AuditLog {
        id, user_id, action: action.to_string(), target_type: target_type.to_string(),
        target_id, details: details.to_string(), created_at: "2026-05-20T00:00:00Z".to_string(),
    });
    kv.put("audit_logs", &logs)?.execute().await?;
    Ok(())
}

// === Crypto ===

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new(); hasher.update(input.as_bytes()); hex::encode(hasher.finalize())
}

fn b64e(input: &str) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let b = input.as_bytes(); let mut o = String::new();
    for ch in b.chunks(3) {
        let a = ch[0] as u32;
        let d = if ch.len() > 1 { ch[1] as u32 } else { 0 };
        let e = if ch.len() > 2 { ch[2] as u32 } else { 0 };
        let n = (a << 16) | (d << 8) | e;
        o.push(T[((n >> 18) & 0x3F) as usize] as char);
        o.push(T[((n >> 12) & 0x3F) as usize] as char);
        if ch.len() > 1 { o.push(T[((n >> 6) & 0x3F) as usize] as char); }
        if ch.len() > 2 { o.push(T[(n & 0x3F) as usize] as char); }
    }
    o
}

fn b64d(input: &str) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes(); let mut result = Vec::new(); let mut i = 0;
    while i < bytes.len() {
        let rem = bytes.len() - i;
        let v0 = T.iter().position(|&c| c == bytes[i]).unwrap_or(0) as u32;
        let v1 = if rem > 1 { T.iter().position(|&c| c == bytes[i+1]).unwrap_or(0) as u32 } else { 0 };
        let v2 = if rem > 2 && bytes[i+2] != b'=' { T.iter().position(|&c| c == bytes[i+2]).unwrap_or(0) as u32 } else { 0 };
        let v3 = if rem > 3 && bytes[i+3] != b'=' { T.iter().position(|&c| c == bytes[i+3]).unwrap_or(0) as u32 } else { 0 };
        let n = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;
        result.push(((n >> 16) & 0xFF) as u8);
        if rem > 2 && bytes[i+2] != b'=' { result.push(((n >> 8) & 0xFF) as u8); }
        if rem > 3 && bytes[i+3] != b'=' { result.push((n & 0xFF) as u8); }
        i += 4;
    }
    String::from_utf8(result).unwrap_or_default()
}

fn make_jwt(payload: &str) -> String {
    let header = b64e(r#"{"alg":"HS256","typ":"JWT"}"#);
    let body = b64e(payload);
    let sig = sha256_hex(&format!("{}.{}", header, body));
    format!("{}.{}.{}", header, body, &sig[..32])
}

fn verify_jwt(token: &str) -> Option<serde_json::Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 { return None; }
    let expected_sig = sha256_hex(&format!("{}.{}", parts[0], parts[1]));
    if &expected_sig[..32] != parts[2] { return None; }
    serde_json::from_str(&b64d(parts[1])).ok()
}

async fn get_auth_user(req: &Request, kv: &KvStore) -> Result<(i64, String)> {
    let auth = req.headers().get("Authorization")?;
    let token = auth
        .and_then(|a| if a.starts_with("Bearer ") { Some(a[7..].to_string()) } else { None })
        .ok_or_else(|| worker::Error::RustError("Unauthorized".into()))?;
    let claims = verify_jwt(&token).ok_or_else(|| worker::Error::RustError("Invalid token".into()))?;
    let username = claims["username"].as_str().unwrap_or("").to_string();
    let users = kv_users(kv).await?;
    let uid = users.iter().find(|u| u.username == username).map(|u| u.id).unwrap_or(0);
    if uid == 0 { return Err(worker::Error::RustError("User not found".into())); }
    Ok((uid, username))
}

fn path_id(req: &Request) -> Result<i64> {
    let url = req.url()?;
    Ok(url.path().rsplit('/').next().unwrap_or("0").parse().unwrap_or(0))
}

async fn gh_fetch(method: &str, url: &str, body: Option<&str>, auth_token: Option<&str>) -> Result<serde_json::Value> {
    let init = web_sys::RequestInit::new();
    init.set_method(method);
    let headers = web_sys::Headers::new().map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    headers.set("Accept", "application/json").map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    headers.set("User-Agent", "apps-cf").map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    if let Some(t) = auth_token {
        headers.set("Authorization", &format!("token {}", t)).map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    }
    if body.is_some() {
        headers.set("Content-Type", "application/x-www-form-urlencoded").map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    }
    init.set_headers(&headers);
    if let Some(b) = body {
        init.set_body(&wasm_bindgen::JsValue::from_str(b));
    }
    let req = web_sys::Request::new_with_str_and_init(url, &init)
        .map_err(|e| worker::Error::RustError(format!("Request init error: {:?}", e)))?;
    let worker_req: worker::Request = req.into();
    let mut resp = worker::Fetch::Request(worker_req).send().await.map_err(|e| worker::Error::RustError(format!("Fetch error: {:?}", e)))?;
    resp.json().await.map_err(|e| worker::Error::RustError(format!("JSON parse error: {:?}", e)))
}

fn build_headers(pairs: &[(&str, &str)]) -> Result<web_sys::Headers> {
    let h = web_sys::Headers::new().map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    for (k, v) in pairs { h.set(*k, *v).map_err(|e| worker::Error::RustError(format!("{:?}", e)))?; }
    Ok(h)
}

fn build_resp(body: Option<&str>, status: u16, headers: &[(&str, &str)]) -> Result<Response> {
    use wasm_bindgen::JsCast;
    let init = web_sys::ResponseInit::new();
    init.set_status(status);
    init.set_headers(build_headers(headers)?.unchecked_ref());
    let resp = web_sys::Response::new_with_opt_str_and_init(body, &init)
        .map_err(|e| worker::Error::RustError(format!("{:?}", e)))?;
    Ok(Response::from(resp))
}

fn json_resp(data: &ApiResponse) -> Result<Response> {
    let body = serde_json::to_string(data)?;
    build_resp(Some(&body), 200, &[("Content-Type","application/json"),("Access-Control-Allow-Origin","*")])
}

fn json_resp_auth(data: &ApiResponse) -> Result<Response> {
    if data.code == 1001 {
        let body = serde_json::to_string(data)?;
        build_resp(Some(&body), 200, &[("Content-Type","application/json"),("Access-Control-Allow-Origin","*"),("WWW-Authenticate","Bearer")])
    } else { json_resp(data) }
}

fn cors_preflight() -> Result<Response> {
    build_resp(None, 204, &[
        ("Access-Control-Allow-Origin","*"),
        ("Access-Control-Allow-Methods","GET,POST,PUT,DELETE,OPTIONS"),
        ("Access-Control-Allow-Headers","Content-Type,Authorization"),
        ("Access-Control-Max-Age","86400"),
    ])
}

fn parse_params(req: &Request) -> Result<Vec<(String, String)>> {
    Ok(req.url()?.query_pairs().map(|(k,v)|(k.to_string(),v.to_string())).collect())
}

// === Main Handler ===

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options { return cors_preflight(); }
    let kv = env.kv("APPS_DATA")?;
    ensure_seeded(&kv).await?;

    let result = Router::new()
        .get_async("/health", |_req, _env| async move {
            json_resp(&ApiResponse::ok(serde_json::json!({"status":"ok","version":"0.1.0"})))
        })
        .get_async("/api/categories", |_req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let cats = kv_categories(&kv).await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&cats)?))
        })
        .get_async("/api/tools", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let params = parse_params(&req)?;
            let mut tools = kv_tools(&kv).await?;
            let page: usize = params.iter().find(|(k,_)|k=="page").and_then(|(_,v)|v.parse().ok()).unwrap_or(1);
            let ps: usize = params.iter().find(|(k,_)|k=="page_size").and_then(|(_,v)|v.parse().ok()).unwrap_or(100);
            if let Some(cid) = params.iter().find(|(k,_)|k=="category_id").and_then(|(_,v)|v.parse::<i64>().ok()) {
                if cid>0 { tools.retain(|t|t.category_id==cid); }
            }
            if let Some(s) = params.iter().find(|(k,_)|k=="search").map(|(_,v)|v.to_lowercase()) {
                if !s.is_empty() { tools.retain(|t|t.name.to_lowercase().contains(&s)||t.description.to_lowercase().contains(&s)); }
            }
            let total = tools.len();
            let items: Vec<_> = tools.into_iter().skip((page-1)*ps).take(ps).collect();
            json_resp(&ApiResponse::ok(serde_json::json!({"items":items,"total":total,"page":page,"page_size":ps})))
        })
        .get_async("/api/stats", |_req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let cats = kv_categories(&kv).await?;
            let tools = kv_tools(&kv).await?;
            let users = kv_users(&kv).await?;
            json_resp(&ApiResponse::ok(serde_json::json!({
                "users": users.len(), "active_users": users.iter().filter(|u|u.is_active).count(),
                "categories": cats.len(), "tools": tools.len(),
                "featured_tools": tools.iter().filter(|t|t.is_featured).count()
            })))
        })
        .post_async("/api/settings/access-password/verify", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let body: PasswordVerify = req.json().await?;
            let pwd: String = kv.get("access_password").text().await?.unwrap_or_default();
            if body.password == pwd { json_resp(&ApiResponse::ok(serde_json::json!({"verified":true}))) }
            else { json_resp(&ApiResponse::err(1002, "Wrong password")) }
        })
        .post_async("/api/auth/login", |mut req, _env| async move {
            let body: LoginRequest = req.json().await?;
            if body.username == ADMIN_USER && body.password == ADMIN_PASSWORD {
                let token = make_jwt(&format!(r#"{{"username":"{}","role":"admin"}}"#, ADMIN_USER));
                json_resp(&ApiResponse::ok(serde_json::json!({"token":token,"user":{"username":ADMIN_USER,"role":"admin"}})))
            } else { json_resp(&ApiResponse::err(1002, "Invalid credentials")) }
        })
        // --- Protected ---
        .get_async("/api/auth/me", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            match get_auth_user(&req, &kv).await {
                Ok((_uid, username)) => {
                    let users = kv_users(&kv).await?;
                    match users.iter().find(|u|u.username==username) {
                        Some(u) => json_resp(&ApiResponse::ok(serde_json::json!({"username":u.username,"role":u.role}))),
                        None => json_resp(&ApiResponse::err(1001, "User not found")),
                    }
                }
                Err(_) => json_resp_auth(&ApiResponse::err(1001, "Unauthorized")),
            }
        })
        // --- Categories CRUD ---
        .post_async("/api/categories", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let input: CategoryInput = req.json().await?;
            let name = input.name.unwrap_or_default();
            if name.is_empty() { return json_resp(&ApiResponse::err(1003, "Category name is required")); }
            let id = alloc_id(&kv).await?;
            let cat = Category { id, name, icon: input.icon.unwrap_or_default(), sort_order: input.sort_order.unwrap_or(0) };
            let mut cats = kv_categories(&kv).await?;
            log_action(&kv, uid, "create", "category", Some(id), &format!("Created category: {}", cat.name)).await?;
            cats.push(cat.clone());
            kv.put("categories", &cats)?.execute().await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&cat)?))
        })
        .put_async("/api/categories/:id", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let input: CategoryInput = req.json().await?;
            let mut cats = kv_categories(&kv).await?;
            let result = match cats.iter_mut().find(|c|c.id==id) {
                Some(cat) => {
                    if let Some(n) = input.name { cat.name = n; }
                    if let Some(i) = input.icon { cat.icon = i; }
                    if let Some(o) = input.sort_order { cat.sort_order = o; }
                    Ok(cat.clone())
                }
                None => Err("Category not found")
            };
            match &result {
                Ok(cat) => { log_action(&kv, uid, "update", "category", Some(id), &format!("Updated category: {}", cat.name)).await?; kv.put("categories", &cats)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(cat) => json_resp(&ApiResponse::ok(serde_json::to_value(&cat)?)), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        .delete_async("/api/categories/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let mut cats = kv_categories(&kv).await?;
            let idx = cats.iter().position(|c|c.id==id);
            match idx {
                Some(i) => { let c = cats.remove(i); log_action(&kv, uid, "delete", "category", Some(id), &format!("Deleted category: {}", c.name)).await?; kv.put("categories", &cats)?.execute().await?; json_resp(&ApiResponse::ok(serde_json::json!({"deleted":true}))) }
                None => json_resp(&ApiResponse::err(1004, "Category not found"))
            }
        })
        // --- Tools CRUD ---
        .get_async("/api/tools/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let id: i64 = path_id(&req)?;
            let tools = kv_tools(&kv).await?;
            match tools.iter().find(|t|t.id==id).cloned() {
                Some(t) => json_resp(&ApiResponse::ok(serde_json::to_value(&t)?)),
                None => json_resp(&ApiResponse::err(1004, "Tool not found"))
            }
        })
        .post_async("/api/tools", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let input: ToolInput = req.json().await?;
            let name = input.name.unwrap_or_default();
            if name.is_empty() { return json_resp(&ApiResponse::err(1003, "Tool name is required")); }
            let id = alloc_id(&kv).await?;
            let tool = Tool {
                id, name, description: input.description.unwrap_or_default(), url: input.url.unwrap_or_default(),
                icon: input.icon.unwrap_or_default(), category_id: input.category_id.unwrap_or(0),
                tags: input.tags.unwrap_or_default(), platforms: input.platforms.unwrap_or_default(),
                versions: input.versions.unwrap_or_default(), sort_order: input.sort_order.unwrap_or(0),
                is_featured: input.is_featured.unwrap_or(false),
            };
            let mut tools = kv_tools(&kv).await?;
            log_action(&kv, uid, "create", "tool", Some(id), &format!("Created tool: {}", tool.name)).await?;
            tools.push(tool.clone());
            kv.put("tools", &tools)?.execute().await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&tool)?))
        })
        .put_async("/api/tools/:id", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let input: ToolInput = req.json().await?;
            let mut tools = kv_tools(&kv).await?;
            let result = match tools.iter_mut().find(|t|t.id==id) {
                Some(tool) => {
                    if let Some(n) = input.name { tool.name = n; }
                    if let Some(d) = input.description { tool.description = d; }
                    if let Some(u) = input.url { tool.url = u; }
                    if let Some(i) = input.icon { tool.icon = i; }
                    if let Some(c) = input.category_id { tool.category_id = c; }
                    if let Some(t) = input.tags { tool.tags = t; }
                    if let Some(p) = input.platforms { tool.platforms = p; }
                    if let Some(v) = input.versions { tool.versions = v; }
                    if let Some(o) = input.sort_order { tool.sort_order = o; }
                    if let Some(f) = input.is_featured { tool.is_featured = f; }
                    Ok(tool.clone())
                }
                None => Err("Tool not found")
            };
            match &result {
                Ok(tool) => { log_action(&kv, uid, "update", "tool", Some(id), &format!("Updated tool: {}", tool.name)).await?; kv.put("tools", &tools)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(tool) => json_resp(&ApiResponse::ok(serde_json::to_value(&tool)?)), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        .delete_async("/api/tools/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let mut tools = kv_tools(&kv).await?;
            let idx = tools.iter().position(|t|t.id==id);
            match idx {
                Some(i) => { let t = tools.remove(i); log_action(&kv, uid, "delete", "tool", Some(id), &format!("Deleted tool: {}", t.name)).await?; kv.put("tools", &tools)?.execute().await?; json_resp(&ApiResponse::ok(serde_json::json!({"deleted":true}))) }
                None => json_resp(&ApiResponse::err(1004, "Tool not found"))
            }
        })
        // --- Users ---
        .get_async("/api/users", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            if let Err(_) = get_auth_user(&req, &kv).await { return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")); }
            let params = parse_params(&req)?;
            let page: usize = params.iter().find(|(k,_)|k=="page").and_then(|(_,v)|v.parse().ok()).unwrap_or(1);
            let ps: usize = params.iter().find(|(k,_)|k=="page_size").and_then(|(_,v)|v.parse().ok()).unwrap_or(20);
            let users = kv_users(&kv).await?;
            let items: Vec<_> = users.iter().map(|u| serde_json::json!({"id":u.id,"username":u.username,"email":u.email,"role":u.role,"is_active":u.is_active})).collect();
            let total = items.len();
            let page_items: Vec<_> = items.into_iter().skip((page-1)*ps).take(ps).collect();
            json_resp(&ApiResponse::ok(serde_json::json!({"items":page_items,"total":total,"page":page,"page_size":ps})))
        })
        .put_async("/api/users/:id", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let input: UserInput = req.json().await?;
            let mut users = kv_users(&kv).await?;
            let result = match users.iter_mut().find(|u|u.id==id) {
                Some(user) => {
                    if let Some(a) = input.is_active { user.is_active = a; }
                    if let Some(r) = input.role { user.role = r; }
                    Ok(serde_json::json!({"id":user.id,"username":user.username.clone(),"email":user.email.clone(),"role":user.role.clone(),"is_active":user.is_active}))
                }
                None => Err("User not found")
            };
            match &result {
                Ok(u) => { let uname = u["username"].as_str().unwrap_or(""); log_action(&kv, uid, "update", "user", Some(id), &format!("Updated user: {}", uname)).await?; kv.put("users", &users)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(u) => json_resp(&ApiResponse::ok(u)), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        .delete_async("/api/users/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let mut users = kv_users(&kv).await?;
            let target = users.iter().find(|u|u.id==id).cloned();
            match target {
                Some(t) => {
                    if t.username == "admin" { return json_resp(&ApiResponse::err(1003, "Cannot delete admin user")); }
                    users.retain(|u|u.id!=id);
                    log_action(&kv, uid, "delete", "user", Some(id), &format!("Deleted user: {}", t.username)).await?;
                    kv.put("users", &users)?.execute().await?;
                    json_resp(&ApiResponse::ok(serde_json::json!({"deleted":true})))
                }
                None => json_resp(&ApiResponse::err(1004, "User not found"))
            }
        })
        // --- Settings ---
        .get_async("/api/settings/access-password", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            if let Err(_) = get_auth_user(&req, &kv).await { return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")); }
            let pwd: String = kv.get("access_password").text().await?.unwrap_or_default();
            json_resp(&ApiResponse::ok(serde_json::json!({"password":pwd})))
        })
        .put_async("/api/settings/access-password", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let body: PasswordUpdate = req.json().await?;
            if body.password.is_empty() { return json_resp(&ApiResponse::err(1003, "Password cannot be empty")); }
            kv.put("access_password", &body.password)?.execute().await?;
            log_action(&kv, uid, "update", "setting", None, "Updated access password").await?;
            json_resp(&ApiResponse::ok(serde_json::json!({"updated":true})))
        })
        // --- System Config ---
        .get_async("/api/settings/system-config", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            if let Err(_) = get_auth_user(&req, &kv).await { return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")); }
            let sc: SystemConfig = kv.get("system_config").json().await?.unwrap_or(SystemConfig { layout: "left".to_string() });
            json_resp(&ApiResponse::ok(serde_json::to_value(&sc)?))
        })
        .put_async("/api/settings/system-config", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let input: SystemConfigInput = req.json().await?;
            let mut sc: SystemConfig = kv.get("system_config").json().await?.unwrap_or(SystemConfig { layout: "left".to_string() });
            if let Some(l) = input.layout {
                if l != "left" && l != "top" { return json_resp(&ApiResponse::err(1003, "layout must be 'left' or 'top'")); }
                sc.layout = l;
            }
            kv.put("system_config", &sc)?.execute().await?;
            log_action(&kv, uid, "update", "system_config", None, &format!("Updated system config layout to {}", sc.layout)).await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&sc)?))
        })
        // --- GitHub OAuth ---
        .get_async("/api/auth/github", |_req, env| async move {
            let client_id = env.var("GITHUB_CLIENT_ID")?.to_string();
            let redirect_uri = format!("{}/api/auth/github/callback", env.var("APP_URL")?.to_string());
            let url = format!("https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email", client_id, redirect_uri);
            build_resp(None, 302, &[("Location",&url)])
        })
        .get_async("/api/auth/github/callback", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let params = parse_params(&req)?;
            let code = params.iter().find(|(k,_)|k=="code").map(|(_,v)|v.to_string()).unwrap_or_default();
            if code.is_empty() { return json_resp(&ApiResponse::err(1003, "Missing authorization code")); }
            let client_id = env.var("GITHUB_CLIENT_ID")?.to_string();
            let client_secret = env.var("GITHUB_CLIENT_SECRET")?.to_string();

            // Step 1: Exchange code for access token (GitHub requires form-urlencoded)
            let token_body = format!("client_id={}&client_secret={}&code={}", client_id, client_secret, code);
            let token_resp: serde_json::Value = match gh_fetch("POST", "https://github.com/login/oauth/access_token", Some(&token_body), None).await {
                Ok(v) => v,
                Err(e) => return json_resp(&ApiResponse::err(1002, &format!("Token exchange error: {}", e))),
            };
            let access_token = token_resp["access_token"].as_str().unwrap_or("").to_string();
            if access_token.is_empty() { return json_resp(&ApiResponse::err(1002, &format!("Token exchange response: {}", token_resp))); }

            // Step 2: Get user profile
            let user_resp: serde_json::Value = gh_fetch("GET", "https://api.github.com/user", None, Some(&access_token)).await?;
            let gh_email = user_resp["email"].as_str().unwrap_or("").to_string();
            let gh_login = user_resp["login"].as_str().unwrap_or("github_user").to_string();

            // Step 3: Get all user emails
            let emails_resp: serde_json::Value = gh_fetch("GET", "https://api.github.com/user/emails", None, Some(&access_token)).await?;
            let mut all_emails = vec![gh_email.clone()];
            if let Some(arr) = emails_resp.as_array() {
                for e in arr { if let Some(email) = e["email"].as_str() { all_emails.push(email.to_string()); } }
            }

            // Check admin whitelist
            let admin_emails: Vec<&str> = vec!["mysinsy@163.com", "scx@polofox.com", "fntp66@gmail.com"];
            let is_admin = all_emails.iter().any(|e| admin_emails.contains(&e.as_str()));
            if !is_admin { return json_resp(&ApiResponse::err(1002, "Access denied: not an admin")); }

            // Create/update user
            let mut users = kv_users(&kv).await?;
            let _uid = match users.iter().find(|u| u.email == gh_login || u.username == gh_login) {
                Some(u) => u.id,
                None => {
                    let new_id = alloc_id(&kv).await?;
                    users.push(User { id: new_id, username: gh_login.clone(), email: gh_email.clone(), password: String::new(), role: "admin".to_string(), is_active: true });
                    kv.put("users", &users)?.execute().await?;
                    new_id
                }
            };
            let token = make_jwt(&format!(r#"{{"username":"{}","role":"admin"}}"#, gh_login));
            let app_url = env.var("APP_URL")?.to_string();
            build_resp(None, 302, &[("Location",&format!("{}/admin?token={}", app_url, token)),("Access-Control-Allow-Origin","*")])
        })
        // --- Audit Logs ---
        .get_async("/api/audit-logs", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            if let Err(_) = get_auth_user(&req, &kv).await { return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")); }
            let params = parse_params(&req)?;
            let page: usize = params.iter().find(|(k,_)|k=="page").and_then(|(_,v)|v.parse().ok()).unwrap_or(1);
            let ps: usize = params.iter().find(|(k,_)|k=="page_size").and_then(|(_,v)|v.parse().ok()).unwrap_or(20);
            let action_filter = params.iter().find(|(k,_)|k=="action").map(|(_,v)|v.to_string());
            let type_filter = params.iter().find(|(k,_)|k=="target_type").map(|(_,v)|v.to_string());
            let mut logs = kv_audit_logs(&kv).await?;
            logs.retain(|l| action_filter.as_ref().map_or(true, |a| l.action == *a));
            logs.retain(|l| type_filter.as_ref().map_or(true, |t| l.target_type == *t));
            logs.sort_by(|a,b| b.id.cmp(&a.id));
            let total = logs.len();
            let items: Vec<_> = logs.into_iter().skip((page-1)*ps).take(ps).collect();
            json_resp(&ApiResponse::ok(serde_json::json!({"items":items,"total":total,"page":page,"page_size":ps})))
        })
        // --- Scripts CRUD ---
        .get_async("/api/scripts", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let params = parse_params(&req)?;
            let mut scripts = kv_scripts(&kv).await?;
            if let Some(tid) = params.iter().find(|(k,_)|k=="tool_id").and_then(|(_,v)|v.parse::<i64>().ok()) {
                scripts.retain(|s|s.tool_id==tid);
            }
            if let Some(p) = params.iter().find(|(k,_)|k=="platform").map(|(_,v)|v.to_lowercase()) {
                if !p.is_empty() { scripts.retain(|s|s.platform.to_lowercase()==p); }
            }
            json_resp(&ApiResponse::ok(serde_json::json!({"items":scripts,"total":scripts.len()})))
        })
        .get_async("/api/scripts/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let id: i64 = path_id(&req)?;
            let scripts = kv_scripts(&kv).await?;
            match scripts.iter().find(|s|s.id==id).cloned() {
                Some(s) => json_resp(&ApiResponse::ok(serde_json::to_value(&s)?)),
                None => json_resp(&ApiResponse::err(1004, "Script not found"))
            }
        })
        .post_async("/api/scripts", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let input: ScriptInput = req.json().await?;
            let tool_id = input.tool_id.unwrap_or(0);
            let name = input.name.unwrap_or_default();
            if tool_id==0 || name.is_empty() { return json_resp(&ApiResponse::err(1003, "tool_id and name are required")); }
            let id = alloc_id(&kv).await?;
            let script = Script {
                id, tool_id, name, content: input.content.unwrap_or_default(),
                platform: input.platform.unwrap_or_default(), tags: input.tags.unwrap_or_default(),
                sort_order: input.sort_order.unwrap_or(0),
            };
            let mut scripts = kv_scripts(&kv).await?;
            log_action(&kv, uid, "create", "script", Some(id), &format!("Created script: {}", script.name)).await?;
            scripts.push(script.clone());
            kv.put("scripts", &scripts)?.execute().await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&script)?))
        })
        .put_async("/api/scripts/:id", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let input: ScriptInput = req.json().await?;
            let mut scripts = kv_scripts(&kv).await?;
            let result = match scripts.iter_mut().find(|s|s.id==id) {
                Some(script) => {
                    if let Some(t) = input.tool_id { script.tool_id = t; }
                    if let Some(n) = input.name { script.name = n; }
                    if let Some(c) = input.content { script.content = c; }
                    if let Some(p) = input.platform { script.platform = p; }
                    if let Some(t) = input.tags { script.tags = t; }
                    if let Some(o) = input.sort_order { script.sort_order = o; }
                    Ok(script.clone())
                }
                None => Err("Script not found")
            };
            match &result {
                Ok(s) => { log_action(&kv, uid, "update", "script", Some(id), &format!("Updated script: {}", s.name)).await?; kv.put("scripts", &scripts)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(s) => json_resp(&ApiResponse::ok(serde_json::to_value(&s)?)), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        .delete_async("/api/scripts/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let mut scripts = kv_scripts(&kv).await?;
            let idx = scripts.iter().position(|s|s.id==id);
            match idx {
                Some(i) => { let s = scripts.remove(i); log_action(&kv, uid, "delete", "script", Some(id), &format!("Deleted script: {}", s.name)).await?; kv.put("scripts", &scripts)?.execute().await?; json_resp(&ApiResponse::ok(serde_json::json!({"deleted":true}))) }
                None => json_resp(&ApiResponse::err(1004, "Script not found"))
            }
        })
        // --- Configs CRUD ---
        .get_async("/api/configs", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let params = parse_params(&req)?;
            let mut configs = kv_configs(&kv).await?;
            if let Some(f) = params.iter().find(|(k,_)|k=="format").map(|(_,v)|v.to_lowercase()) {
                if !f.is_empty() { configs.retain(|c|c.format.to_lowercase()==f); }
            }
            json_resp(&ApiResponse::ok(serde_json::json!({"items":configs,"total":configs.len()})))
        })
        .get_async("/api/configs/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let id: i64 = path_id(&req)?;
            let configs = kv_configs(&kv).await?;
            match configs.iter().find(|c|c.id==id).cloned() {
                Some(c) => json_resp(&ApiResponse::ok(serde_json::to_value(&c)?)),
                None => json_resp(&ApiResponse::err(1004, "Config not found"))
            }
        })
        .post_async("/api/configs", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let input: ConfigInput = req.json().await?;
            let name = input.name.unwrap_or_default();
            if name.is_empty() { return json_resp(&ApiResponse::err(1003, "Config name is required")); }
            let id = alloc_id(&kv).await?;
            let config = Config {
                id, name, format: input.format.unwrap_or_else(||"text".to_string()),
                content: input.content.unwrap_or_default(), sort_order: input.sort_order.unwrap_or(0),
                copy_count: 0,
            };
            let mut configs = kv_configs(&kv).await?;
            log_action(&kv, uid, "create", "config", Some(id), &format!("Created config: {}", config.name)).await?;
            configs.push(config.clone());
            kv.put("configs", &configs)?.execute().await?;
            json_resp(&ApiResponse::ok(serde_json::to_value(&config)?))
        })
        .put_async("/api/configs/:id", |mut req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let input: ConfigInput = req.json().await?;
            let mut configs = kv_configs(&kv).await?;
            let result = match configs.iter_mut().find(|c|c.id==id) {
                Some(config) => {
                    if let Some(n) = input.name { config.name = n; }
                    if let Some(f) = input.format { config.format = f; }
                    if let Some(c) = input.content { config.content = c; }
                    if let Some(o) = input.sort_order { config.sort_order = o; }
                    Ok(config.clone())
                }
                None => Err("Config not found")
            };
            match &result {
                Ok(c) => { log_action(&kv, uid, "update", "config", Some(id), &format!("Updated config: {}", c.name)).await?; kv.put("configs", &configs)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(c) => json_resp(&ApiResponse::ok(serde_json::to_value(&c)?)), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        .delete_async("/api/configs/:id", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let (uid, _) = match get_auth_user(&req, &kv).await { Ok(v) => v, Err(_) => return json_resp_auth(&ApiResponse::err(1001, "Unauthorized")) };
            let id: i64 = path_id(&req)?;
            let mut configs = kv_configs(&kv).await?;
            let idx = configs.iter().position(|c|c.id==id);
            match idx {
                Some(i) => { let c = configs.remove(i); log_action(&kv, uid, "delete", "config", Some(id), &format!("Deleted config: {}", c.name)).await?; kv.put("configs", &configs)?.execute().await?; json_resp(&ApiResponse::ok(serde_json::json!({"deleted":true}))) }
                None => json_resp(&ApiResponse::err(1004, "Config not found"))
            }
        })
        .post_async("/api/configs/:id/copy", |req, env| async move {
            let kv = env.kv("APPS_DATA")?;
            let id: i64 = path_id(&req)?;
            let mut configs = kv_configs(&kv).await?;
            let result = match configs.iter_mut().find(|c|c.id==id) {
                Some(config) => { config.copy_count += 1; Ok(config.clone()) }
                None => Err("Config not found")
            };
            match &result {
                Ok(_) => { kv.put("configs", &configs)?.execute().await?; }
                Err(_) => {}
            }
            match result { Ok(c) => json_resp(&ApiResponse::ok(serde_json::json!({"copy_count":c.copy_count}))), Err(msg) => json_resp(&ApiResponse::err(1004, msg)) }
        })
        // --- SPA Fallback ---
        .get_async("/*path", |_req, _env| async move {
            build_resp(Some(INDEX_HTML), 200, &[("Content-Type","text/html; charset=utf-8")])
        })
        .run(req, env)
        .await;

    match result {
        Ok(resp) => Ok(resp),
        Err(e) => json_resp(&ApiResponse::err(500, &format!("{}", e))),
    }
}
