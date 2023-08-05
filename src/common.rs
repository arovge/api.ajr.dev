use worker::*;

const API_KEY_VAR: &str = "API_KEY";
const API_KEY_HEADER: &str = "X-API-Key";

pub fn ok() -> Result<Response> {
    Response::ok("OK")
}

pub async fn ok_with_file(file: Object) -> Result<Response> {
    let Some(body) = file.body() else {
        return internal_server_error();
    };
    let bytes = body.bytes().await?;
    let mut headers = Headers::new();
    headers.append("etag", &file.http_etag())?;
    let response = Response::from_bytes(bytes)?.with_headers(headers);
    Ok(response)
}

pub fn not_found() -> Result<Response> {
    let mut headers = Headers::new();

    // If the client is requesting a *.jpeg (or other file types), Firefox would still try to show the image with:
    // `The image cannot be displayed because it contains errors`
    // Force the content type to text/plain to show the 404 message for any web browsers that do this
    headers.append("Content-Type", "text/plain")?;
    let response = Response::error("404s and heartbreaks", 404)
        .unwrap()
        .with_headers(headers);
    Ok(response)
}

pub fn forbidden() -> Result<Response> {
    Response::error("Forbidden", 405)
}

pub fn internal_server_error() -> Result<Response> {
    Response::error("Internal Server Error", 500)
}

pub fn is_authorized(req: &Request, env: &Env) -> Result<bool> {
    let api_key = env.var(API_KEY_VAR)?.to_string();
    let is_authorized = req.headers().get(API_KEY_HEADER)? == Some(api_key);
    Ok(is_authorized)
}
