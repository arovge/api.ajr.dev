use worker::*;

mod common;
mod files;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // TODO: Match path
    // TODO: Get key from path
    // TODO: Authorization for put/delete
    
    let api_key = env.var("X-API-Key")?.to_string();
    let is_authorized = req.headers().get("X-API-Key")? == Some(api_key);

    match req {
        req if req.method() == Method::Get => {
            let file = files::get_file(&env, "key").await?;
            let Some(obj) = file else { return common::not_found() };
            let txt = obj.body().unwrap().text().await?;
            common::ok_with_body(&txt)
        },
        mut req if req.method() == Method::Put => {
            let body = req.bytes().await?;
            files::put_file(&env, "key", body).await?;
            common::ok()
        },
        req if req.method() == Method::Delete => {
            files::delete_file(&env, "key").await?;
            common::ok()
        },
        _ => common::not_found()
    }
}
