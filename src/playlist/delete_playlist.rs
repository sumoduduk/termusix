use super::Playlist;

pub fn remove_playlist(playlist: &mut Playlist, index: usize) -> Option<()> {
    playlist.0.swap_remove_index(index)?;
    Some(())
}

pub fn remove_song(playlist: &mut Playlist, index_id: usize, index_song: usize) -> Option<()> {
    let playlist_info = playlist.0.get_index_mut(index_id)?;
    playlist_info.1.music_list.shift_remove_index(index_song)?;

    Some(())
}
