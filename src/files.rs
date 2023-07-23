use worker::*;

const FILES_BUCKET: &str = "FILES_BUCKET";

pub async fn get_file(env: &Env, key: &str) -> Result<Option<Object>> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.get(key).execute().await
}

pub async fn put_file(env: &Env, key: &str, body: Vec<u8>) -> Result<Object> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.put(key, body).execute().await
}

pub async fn delete_file(env: &Env, key: &str) -> Result<()> {
    let bucket = env.bucket(FILES_BUCKET)?;
    bucket.delete(key).await
}
