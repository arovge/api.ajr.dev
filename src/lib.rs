use worker::*;

mod common;
mod files;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {

    // TODO: Look into axum and see if they have a generic routing solution to replace this
    // Or create one (and learn about rust macros! maybe do that anyway just to learn)
    if !req.path().starts_with("/files/") { return common::not_found(); }
    let key = req.path().replace("/files/", "");
    if key.is_empty() { return common::not_found(); }

    match req {
        req if req.method() == Method::Get => {
            files::get_file_handler(env, &key).await
        },
        req if req.method() == Method::Put => {
            files::put_file_handler(req, env, &key).await
        },
        req if req.method() == Method::Delete => {
            files::delete_file_handler(req, env, &key).await
        },
        _ => common::not_found()
    }
}
