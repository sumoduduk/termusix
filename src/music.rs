use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::{default::Default, path::Path};

pub async fn download_music(id: &str) -> eyre::Result<()> {
    let opt = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(id, opt)?;

    let path = Path::new("music/test.mp3");

    let res = video.download(path).await;
    dbg!(res);

    // println!("success download to : {}", video.display());

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const ID: &str = "Rgszfmk7ti0";

    #[tokio::test]
    async fn test_download_1() -> eyre::Result<()> {
        let res = download_music(ID).await;
        dbg!(&res);

        assert!(res.is_ok());
        Ok(())
    }
}
