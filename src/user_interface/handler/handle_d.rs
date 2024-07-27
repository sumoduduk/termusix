use crate::app::{screen::Screen, App};

pub fn handle_delete_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            if let Some(playlist_index) = app.tabs_playlist.selected() {
                app.playlist.delete_playlist(playlist_index);
            }
        }

        Screen::ListMusic => {
            if let Some(playlist_index) = app.tabs_playlist.selected() {
                if let Some(song_index) = app.music_list.selected() {
                    app.playlist.delete_song(playlist_index, song_index);
                }
            }
        }

        _ => {}
    }
}
