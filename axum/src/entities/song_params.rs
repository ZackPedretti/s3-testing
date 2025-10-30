use serde::Deserialize;
#[derive(Deserialize)]
pub struct SongParams {
    pub artist: String,
    pub song: String,
    pub extension: Option<String>,
}
