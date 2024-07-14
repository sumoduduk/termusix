mod list_playlist;
mod save_playlist;

use save_playlist::{get_playlist, save_file_json};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MusicInfo {
    music_id: String,
    music_title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MusicPlaylist {
    playlist_id: String,
    playlis_title: String,
    music_list: Vec<MusicInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    music_playlist: Vec<MusicPlaylist>,
}

impl Playlist {
    pub fn new() -> eyre::Result<Self> {
        let raw = fs::read_to_string("music.json")?;

        let data: Vec<MusicPlaylist> = serde_json::from_str(&raw)?;

        let playlist = Playlist {
            music_playlist: data,
        };

        Ok(playlist)
    }

    pub async fn save_playlist(&mut self, url: &str) -> eyre::Result<()> {
        let extract_playlist = get_playlist(url).await?;
        self.music_playlist.push(extract_playlist);
        save_file_json(&self.music_playlist)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init_playlist() {
        let struc_data = Playlist::new();
        dbg!(&struc_data);

        assert!(struc_data.is_ok());
    }
}
