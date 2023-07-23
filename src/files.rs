use worker::*;
use crate::common;

const FILES_BUCKET: &str = "FILES_BUCKET";

pub async fn get_file_handler(env: Env, key: &str) -> Result<Response> {
    let file = get_file(&env, key).await?;
    let Some(obj) = file else { return common::not_found(); };
    let txt = obj.body().unwrap().text().await?;
    common::ok_with_body(&txt)
}

pub async fn put_file_handler(mut req: Request, env: Env, key: &str) -> Result<Response> {
    if !common::is_authorized(&req, &env)? {
        return common::forbidden();
    }

    let body = req.bytes().await?;
    put_file(&env, key, body).await?;
    common::ok()
}

pub async fn delete_file_handler(req: Request, env: Env, key: &str) -> Result<Response> {
    if !common::is_authorized(&req, &env)? {
        return common::forbidden();
    }

    delete_file(&env, key).await?;
    common::ok()
}

async fn get_file(env: &Env, key: &str) -> Result<Option<Object>> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.get(key).execute().await
}

async fn put_file(env: &Env, key: &str, body: Vec<u8>) -> Result<Object> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.put(key, body).execute().await
}

async fn delete_file(env: &Env, key: &str) -> Result<()> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.delete(key).await
}
