use eyre::{eyre, Context};
use rodio::Decoder;
use std::{fs::File, path::Path, sync::mpsc::Sender};
use tokio::fs::read_dir;

use crate::playback::PlaybackEvent;

pub async fn send_id_file_exist(
    id: &str,
    music_dir: &Path,
    sender: Sender<PlaybackEvent>,
) -> eyre::Result<()> {
    let mut entries = read_dir(music_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        if let Some(path_file_name) = entry.file_name().to_str() {
            if path_file_name.contains(id) {
                let _ = sender.send(PlaybackEvent::Add(entry.path()));
            }
        }
    }
    Ok(())
}

pub fn decode_file(path: &Path) -> eyre::Result<Decoder<File>> {
    if let Ok(file) = File::open(path) {
        let decoder = match extract_extentions(path) {
            Some("mp4") => rodio::Decoder::new_mp4(file, rodio::decoder::Mp4Type::M4a)
                .wrap_err("cant decode file"),
            Some("mp3") | Some("flac") | Some("wav") | Some("ogg") => {
                rodio::Decoder::new(file).wrap_err("cant decode file")
            }
            _ => Err(eyre!("ERROR: not sound codec")),
        };

        decoder
    } else {
        Err(eyre!("ERROR: cant open file"))
    }
}

pub fn extract_extentions(path: &Path) -> Option<&str> {
    path.extension().and_then(|ext| ext.to_str())
}

pub fn get_parent(path: &Path) -> Option<&Path> {
    path.parent()
}
