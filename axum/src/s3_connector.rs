use axum::body::Bytes;
use std::error::Error;

use crate::entities::song_params::SongParams;

pub async fn put_song(params: SongParams, file: Bytes) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn get_song(params: SongParams) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(vec![])
}
