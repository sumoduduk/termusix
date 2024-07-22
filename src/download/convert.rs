use std::process::Output;

use tokio::fs;
use tokio::process::Command;

pub async fn convert_codec(video_id: &str) -> eyre::Result<Output> {
    let in_file = format!("music/{}.webm", video_id);
    let out_file = format!("music/{}.mp3", video_id);

    let cmd_dest = if cfg!(unix) {
        String::from("ffmpeg")
    } else {
        String::from("./data/ffmpeg")
    };

    let output = Command::new(cmd_dest)
        .args([
            "-i", &in_file, "-vn", "-ar", "44100", "-ac", "2", "-b:a", "192k", &out_file,
        ])
        .output()
        .await?;

    fs::remove_file(in_file).await.unwrap();

    Ok(output)
}
