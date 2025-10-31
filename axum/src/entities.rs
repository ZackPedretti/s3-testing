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

impl SongParams {
    pub fn build_key(&self) -> String {
        format!(
            "{}/{}.{}",
            self.artist,
            self.song,
            self.extension.clone().unwrap_or("mp3".to_string())
        )
    }

    pub fn build_filename(&self) -> String {
        format!(
            "{}.{}",
            self.song,
            self.extension.clone().unwrap_or("mp3".to_string())
        )
    }
}
