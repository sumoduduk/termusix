mod save_playlist;

use eyre::OptionExt;
use indexmap::IndexMap;
use save_playlist::{get_playlist, save_file_json, save_id};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::utils::shuffle_vec;

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// struct MusicInfo {
//     music_id: String,
//     music_title: String,
// }

type MusicInfo = IndexMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct InfoMusicPlaylist {
    playlist_title: String,
    music_list: MusicInfo,
}

type MusicPlaylist = IndexMap<String, InfoMusicPlaylist>;

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

    pub fn list_playlist_titles(&self) -> Vec<String> {
        let music_playlist = &self.0;

        let titles: Vec<String> = music_playlist
            .values()
            .map(|t| t.playlist_title.to_owned())
            .collect();
        titles
    }

    pub fn list_music_by_idx(&mut self, indx: Option<usize>) -> Option<MusicInfo> {
        let Some(indx) = indx else { return None };
        let Some(info_playlist) = self.0.get_index(indx) else {
            return None;
        };

        let music = &info_playlist.1.music_list;
        Some(music.clone())
    }

    pub fn list_shuffled_music_id(&self, id: &str) -> eyre::Result<Vec<String>> {
        let music_playlist = &self.0;

        let info_playlist = music_playlist
            .get(id)
            .ok_or_eyre("ERROR: Id playlist not found")?;

        let mut music_ids: Vec<String> =
            info_playlist.music_list.keys().map(|s| s.into()).collect();
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
