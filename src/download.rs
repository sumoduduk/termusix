mod convert;

use convert::convert_codec;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::{path::Path, process::Output};

pub async fn download_music(id: &str) -> eyre::Result<Output> {
    let path_file = ["music/", id, ".webm"].concat();

    let opt = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(id, opt)?;

    let path = Path::new(&path_file);

    video.download(path).await?;

    let output = convert_codec(id).await?;
    Ok(output)
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
