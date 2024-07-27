use std::path::PathBuf;

use crate::utils::time_now;

use super::{InfoMusicPlaylist, Playlist};
use indexmap::IndexMap;

pub fn save_local(playlist: &mut Playlist, id: &str) {
    let music_info = IndexMap::new();
    let info_playlist: InfoMusicPlaylist = InfoMusicPlaylist {
        playlist_title: id.to_owned(),
        music_list: music_info,
    };

    let time = time_now();

    let key = [id, "-local", "-", &time.to_string()].concat();

    playlist.0.insert(key, info_playlist);
}

pub fn save_local_music(
    playlist: &mut Playlist,
    song_list: Vec<PathBuf>,
    index_id: usize,
) -> Option<()> {
    for song in song_list {
        let os_name = song.file_stem().map(|s| s.to_str())??;
        let music_info = playlist.0.get_index_mut(index_id)?;
        music_info
            .1
            .music_list
            .insert(song.to_str()?.to_owned(), os_name.to_owned());
    }

    Some(())
}
