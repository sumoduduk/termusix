use crate::app::{screen::Screen, App};

pub async fn enter_key(app: &mut App) {
    match app.screen_state {
        Screen::InsertPlaylist => {
            ui_save_playlit(app).await;
        }
        Screen::Playlist => {
            app.screen_state = Screen::ListMusic;
        }
        _ => {}
    }
}

async fn ui_save_playlit(app: &mut App) {
    let playlist_input = app.input_playlist.value();

    if playlist_input.contains("youtube") && playlist_input.contains("playlist") {
        let _ = app.playlist.save_playlist(playlist_input).await;
    } else {
        let _ = app.playlist.save_local_playlist(playlist_input);
    }

    app.input_playlist.reset();
    app.tabs_playlist.select_last();
    app.screen_state = Screen::ListMusic;
}
