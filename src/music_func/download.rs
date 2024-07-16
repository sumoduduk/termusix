use std::path::Path;

use eyre::Context;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
pub async fn download_music(id: &str, path_file: &str) -> eyre::Result<()> {
    println!("begin download music to : {}", path_file);

    let opt = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(id, opt)?;

    let path = Path::new(path_file);

    let res = video
        .download(path)
        .await
        .wrap_err_with(|| format!("ERROR: Downloading music in {}", path_file));

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    const ID: &str = "Rgszfmk7ti0";

    #[tokio::test]
    async fn test_download_1() -> eyre::Result<()> {
        let res = download_music(ID, "music/Rgszfmk7ti0.mp3").await;
        dbg!(&res);

        assert!(res.is_ok());
        Ok(())
    }
}
