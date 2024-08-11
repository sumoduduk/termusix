mod delete_playlist;
mod save_local;
mod save_playlist;

use delete_playlist::{remove_playlist, remove_song};
use indexmap::IndexMap;
use save_local::{save_local, save_local_music};
use save_playlist::{get_playlist, save_file_json, save_id};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

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
pub struct Playlist(MusicPlaylist, PathBuf);

impl Playlist {
    pub fn new() -> eyre::Result<Self> {
        let dir_config = dirs::config_local_dir()
            .expect("ERROR: No local config found")
            .join("termusix");

        if !dir_config.exists() {
            let _ = create_dir_all(&dir_config);
        }

        let conf_file = dir_config.join("music.json");

        if !conf_file.exists() {
            Self::create_config_file(&conf_file).expect("Create Config File Failed");
        }

        let raw = fs::read_to_string(&conf_file)?;
        let data: MusicPlaylist = serde_json::from_str(&raw)?;
        let playlist = Playlist(data, conf_file);

        Ok(playlist)
    }

    pub async fn save_playlist(&mut self, url: &str) -> eyre::Result<()> {
        get_playlist(self, url).await?;
        save_file_json(&self.0, &self.1)?;
        Ok(())
    }

    pub fn save_local_playlist(&mut self, id: &str) -> eyre::Result<()> {
        save_local(self, id);

        save_file_json(&self.0, &self.1)?;
        Ok(())
    }

    pub fn save_local_song(&mut self, song_list: &[PathBuf], index_id: usize) -> eyre::Result<()> {
        save_local_music(self, song_list, index_id);

        save_file_json(&self.0, &self.1)?;
        Ok(())
    }

    fn create_config_file(path_file: &Path) -> eyre::Result<()> {
        fs::write(path_file, "{}")?;
        Ok(())
    }

    pub async fn save_by_id(&mut self, id: &str) -> eyre::Result<()> {
        save_id(self, id).await?;
        save_file_json(&self.0, &self.1)?;
        Ok(())
    }

    pub fn delete_playlist(&mut self, index_id: usize) {
        if remove_playlist(self, index_id).is_some() {
            let _ = save_file_json(&self.0, &self.1);
        }
    }

    pub fn delete_song(&mut self, index_id: usize, index_song: usize) {
        if remove_song(self, index_id, index_song).is_some() {
            let _ = save_file_json(&self.0, &self.1);
        }
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

    pub fn list_music_by_idx(&self, indx: Option<usize>) -> Option<&MusicInfo> {
        let indx = indx?;
        let info_playlist = self.0.get_index(indx)?;

        let music = &info_playlist.1.music_list;
        Some(music)
    }

    pub fn get_playlist_title(&self, index: Option<usize>) -> Option<&str> {
        let (_, info) = self.0.get_index(index?)?;

        Some(&info.playlist_title)
    }

    pub fn get_playlist_id(&self, index: Option<usize>) -> Option<&str> {
        let (pl_id, _) = self.0.get_index(index?)?;

        Some(pl_id)
    }

    pub fn get_music_title(
        &self,
        index_playlist: Option<usize>,
        index_song: Option<usize>,
    ) -> Option<&str> {
        let (_, info) = self.0.get_index(index_playlist?)?;
        let (_, title) = info.music_list.get_index(index_song?)?;

        Some(title)
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
