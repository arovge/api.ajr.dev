use worker::*;

const API_KEY_ENVVAR: &str = "SHARED_API_KEY";
const API_KEY_HEADER: &str = "X-API-Key";

pub fn ok() -> Result<Response> {
    Response::ok("OK")
}

pub fn ok_with_body(body: &str) -> Result<Response> {
    Response::ok(body)
}

pub fn not_found() -> Result<Response> {
    Response::error("404s and heartbreaks", 404)
}

pub fn forbidden() -> Result<Response> {
    Response::error("Forbidden", 405)
}

pub fn is_authorized(req: &Request, env: &Env) -> Result<bool> {
    let api_key = env.var(API_KEY_ENVVAR)?.to_string();
    let is_authorized = req.headers().get(API_KEY_HEADER)? == Some(api_key);
    Ok(is_authorized)
}
