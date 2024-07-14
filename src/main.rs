mod music;
mod playlist;

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

    let id = args.next().unwrap();

    let instance = "https://inv.tux.pizza";

    let url = format!("{}/api/v1/playlists/{}", instance, id);

    let data_json: PlaylistInfo = reqwest::get(&url).await?.json().await?;
    let pretty_json = serde_json::to_string_pretty(&data_json)?;
    println!("{}", pretty_json);

    Ok(())
}
