use aws_sdk_s3 as s3;
use serde::Deserialize;

#[derive(Clone)]
pub struct ApiState {
    pub s3_client: s3::Client,
}

#[derive(Deserialize, Debug)]
pub struct SongParams {
    pub artist: String,
    pub song: String,
    pub extension: Option<String>,
}
