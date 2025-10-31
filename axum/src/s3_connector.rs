use aws_sdk_s3 as s3;
use axum::body::Bytes;

use crate::entities::SongParams;

pub async fn init_client() -> anyhow::Result<s3::Client> {
    let config = aws_config::load_from_env().await;

    let client = s3::Client::new(&config);

    match client.list_buckets().send().await {
        Ok(_) => Ok(client),
        Err(e) => Err(e.into()),
    }
}

pub async fn put_song(params: SongParams, file: Bytes, client: &s3::Client) -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_song(params: SongParams, client: &s3::Client) -> anyhow::Result<Vec<u8>> {
    Ok(vec![])
}
