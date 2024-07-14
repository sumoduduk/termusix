mod music;
mod playlist;

use core::panic;
use playlist::Playlist;
use serde::{Deserialize, Serialize};
use std::env::args;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoResult {
    title: String,
    video_id: String,
    author: String,
    length_seconds: usize,
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    title: String,
    playlist_id: String,
    author: String,
    video_count: usize,
    videos: Vec<VideoResult>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut args = args();
    args.next();

    let arg = args
        .next()
        .expect("ERROR: argument expected => --save, --list");

    dbg!(&arg);

    let mut playlist = Playlist::new()?;

    match arg.trim() {
        "--save" => {
            let url = args.next().expect("ERROR: provide the URL");
            playlist.save_playlist(&url).await?;
            println!("Playlist saved..")
        }
        "--list" => todo!(),

        _ => println!("Please provide correct argument, --save, --list"),
    }

    Ok(())
}
