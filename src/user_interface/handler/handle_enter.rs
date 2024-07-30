use crate::app::{screen::Screen, App};

pub async fn enter_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.screen_state = Screen::ListMusic;
        }
        Screen::PopUpFileExplorer => {
            app.save_song_to_playlist();
            app.screen_state = Screen::ListMusic;
        }
        _ => {}
    }
}
