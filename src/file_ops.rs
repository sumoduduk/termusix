use eyre::{eyre, Context};
use rodio::Decoder;
use std::{
    fs::{metadata as metada_sync, File},
    path::Path,
    sync::mpsc::Sender,
};
use tokio::fs::{metadata, read_dir};

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
            Some("mp3") => rodio::Decoder::new(file).wrap_err("cant decode file"),
            _ => Err(eyre!("ERROR: not mp3 and mp4 file")),
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

pub async fn check_file_exist(file_name: &str) -> Option<&str> {
    let meta_res = metadata(file_name).await;
    match meta_res {
        Ok(meta_res) => {
            let size = meta_res.len();
            let is_file = meta_res.is_file();

            if size > 0 && is_file {
                Some(file_name)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn check_file_exist_sync(file_name: &str) -> Option<&str> {
    let meta_res = metada_sync(file_name);
    match meta_res {
        Ok(meta_res) => {
            let size = meta_res.len();
            let is_file = meta_res.is_file();

            if size > 0 && is_file {
                Some(file_name)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

// pub async fn count_file(folder_path: &str) -> eyre::Result<usize> {
//     let mut count = 0;
//
//     let mut dir = fs::read_dir(folder_path).await?;
//     while let Some(entry) = dir.next_entry().await? {
//         let file_type = entry.file_type().await?;
//         if file_type.is_file() {
//             count += 1;
//         }
//     }
//
//     Ok(count)
// }
