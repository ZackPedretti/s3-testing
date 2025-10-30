use std::error::Error;

pub async fn put_song(
    artist: String,
    song: String,
    extension: Option<String>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn get_song(
    artist: String,
    song: String,
    extension: Option<String>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(vec![])
}
