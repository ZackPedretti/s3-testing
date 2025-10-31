use aws_sdk_s3 as s3;

use crate::entities::SongParams;

pub async fn init_client() -> anyhow::Result<s3::Client> {
    let config = aws_config::load_from_env().await;

    let client = s3::Client::new(&config);

    match client.list_buckets().send().await {
        Ok(_) => Ok(client),
        Err(e) => Err(e.into()),
    }
}

pub async fn put_song(
    params: &SongParams,
    file: axum::body::Bytes,
    client: &s3::Client,
) -> anyhow::Result<()> {
    client
        .put_object()
        .bucket("jukebox")
        .key(params.build_key())
        .body(file.into())
        .send()
        .await?;
    Ok(())
}

pub async fn get_song(params: &SongParams, client: &s3::Client) -> anyhow::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    let mut object = client
        .get_object()
        .bucket("jukebox")
        .key(params.build_key())
        .send()
        .await?;

    while let Some(bytes) = object.body.try_next().await? {
        buffer.extend_from_slice(&bytes);
    }

    Ok(buffer)
}
