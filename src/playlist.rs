mod list_music;
mod save_playlist;

use eyre::OptionExt;
use save_playlist::{get_playlist, save_file_json, save_id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::utils::shuffle_vec;

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// struct MusicInfo {
//     music_id: String,
//     music_title: String,
// }

type MusicInfo = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct InfoMusicPlaylist {
    playlist_title: String,
    music_list: MusicInfo,
}

type MusicPlaylist = HashMap<String, InfoMusicPlaylist>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist(MusicPlaylist);

impl Playlist {
    pub fn new() -> eyre::Result<Self> {
        let raw = fs::read_to_string("music.json")?;

        let data: MusicPlaylist = serde_json::from_str(&raw)?;

        let playlist = Playlist(data);

        Ok(playlist)
    }

    pub async fn save_playlist(&mut self, url: &str) -> eyre::Result<()> {
        get_playlist(self, url).await?;
        save_file_json(&self.0)?;
        Ok(())
    }

    pub async fn save_by_id(&mut self, id: &str) -> eyre::Result<()> {
        save_id(self, id).await?;
        save_file_json(&self.0)?;
        Ok(())
    }

    pub fn list_playlist(&self) {
        let music_playlist = &self.0;

        for (id, info_playlist) in music_playlist {
            let title = &info_playlist.playlist_title;

            println!("{id} - {title}");
        }
    }

    pub fn list_shuffled_music_id(&self, id: &str) -> eyre::Result<Vec<&String>> {
        let music_playlist = &self.0;

        let info_playlist = music_playlist
            .get(id)
            .ok_or_eyre("ERROR: Id playlist not found")?;

        let mut music_ids: Vec<&String> = info_playlist.music_list.keys().collect();
        shuffle_vec(&mut music_ids);

        Ok(music_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_playlist() {
        let struc_data = Playlist::new();
        dbg!(&struc_data);

        assert!(struc_data.is_ok());
    }
}
