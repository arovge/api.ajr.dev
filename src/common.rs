use worker::*;

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
