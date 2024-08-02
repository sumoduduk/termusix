use std::path::Path;
use std::process::Output;

use eyre::OptionExt;
use tokio::fs::{self, read_dir};
use tokio::process::Command;

use crate::file_ops::{extract_extentions, get_parent};

pub async fn convert_folder() -> eyre::Result<()> {
    let home = dirs::home_dir().expect("need home dir");
    let music_dir = dirs::audio_dir().unwrap_or(home.join("Music"));
    let mut entries = read_dir(music_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path_buf = entry.path();
        let ext = extract_extentions(&path_buf).ok_or_eyre("cant get extenstion")?;
        if ext == "mp4" {
            let res = convert_codec(&path_buf).await;

            let path_display = &path_buf.display();

            if res.is_err() {
                println!("ERROR: failed to conver to mp3 : {path_display}")
            } else {
                println!("{path_display} sucessfully converted.")
            }
        }
    }

    Ok(())
}

async fn convert_codec(mp4_path: &Path) -> eyre::Result<Output> {
    let in_file = mp4_path.to_str().ok_or_eyre("Cant get str of in_file")?;

    let parent = get_parent(mp4_path).ok_or_eyre("cant get parent of infile")?;

    let stem_name = mp4_path
        .file_stem()
        .and_then(|f| f.to_str())
        .ok_or_eyre("cant get stem name of in file")?;

    let file_name = [stem_name, ".mp3"].concat();

    let out_file_path = parent.join(file_name);

    let out_file = out_file_path
        .to_str()
        .ok_or_eyre("cant get str from out_file_path")?;

    println!("converting to : {}", out_file);

    let cmd_dest = if cfg!(unix) {
        String::from("ffmpeg")
    } else {
        String::from("./data/ffmpeg")
    };

    let output = Command::new(cmd_dest)
        .args([
            "-i", in_file, "-vn", "-ar", "44100", "-ac", "2", "-b:a", "192k", out_file,
        ])
        .output()
        .await?;

    fs::remove_file(in_file).await.unwrap();

    Ok(output)
}
