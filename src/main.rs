mod file_ops;
mod music_func;
mod playlist;
mod utils;

use music_func::play;
use playlist::Playlist;
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::env::args;
use std::{fs::File, io::BufReader};
use tokio::sync::mpsc;

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
        .expect("ERROR: argument expected => --save, --list, --play");

    dbg!(&arg);

    let mut playlist = Playlist::new()?;

    match arg.trim() {
        "--save" => {
            let url = args.next().expect("ERROR: provide the URL");
            playlist.save_playlist(&url).await?;
            println!("Playlist saved..")
        }
        "--save-id" => {
            let id = args.next().expect("ERROR: provide the ID");
            playlist.save_by_id(&id).await?;
            println!("Playlist ID saved..")
        }
        "--list" => {
            playlist.list_playlist();
        }

        "--play" => {
            let id = args.next().expect("ERROR: provide the ID");
            let list_id = playlist.list_shuffled_music_id(&id);

            let (tx, mut rx) = mpsc::channel(10);
            match list_id {
                Ok(list) => play(list, tx).await?,
                Err(error) => println!("{error}"),
            }

            let (_, stram_handle) =
                OutputStream::try_default().expect("ERROR: error getting OutputStream");
            let sink = Sink::try_new(&stram_handle).expect("ERROR: play new sink");

            while let Some(music_file) = rx.blocking_recv() {
                let file_raw =
                    File::open(&music_file).expect("ERROR: can't open a file in {music_file}");
                let file = BufReader::new(file_raw);

                if let Ok(source) = rodio::Decoder::new(file) {
                    sink.append(source);
                } else {
                    eprintln!("ERROR: can't add {} into playlist", &music_file);
                }
            }
        }

        _ => println!("Please provide correct argument, --save, --list, --play"),
    }

    Ok(())
}
