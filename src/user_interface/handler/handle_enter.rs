use crate::app::{screen::Screen, App};

pub fn enter_key(app: &mut App) {
    match app.screen_state {
        Screen::InsertPlaylist => {
            app.input_playlist.reset();
        }
        Screen::Playlist => {
            app.screen_state = Screen::ListMusic;
        }
        _ => {}
    }
}
