use crate::{app::{screen::Screen, App}, playback::PlaybackEvent};

pub async fn enter_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.screen_state = Screen::ListMusic;
        }
        Screen::PopUpFileExplorer => {
            app.save_song_to_playlist();
            app.screen_state = Screen::ListMusic;
        }
        Screen::ListMusic => {
            let Some(song_id) = app.music_list.selected() else {
                return;
            };

            let sender = app.tx_playback.clone();
            let _ = sender.send(PlaybackEvent::TrackPlay())

            if let Ok(now_playing) = app.now_playing.read() {
                match now_playing.playlist_id {
                    Some(id) => {
                        if let Some(index) = app.tabs_playlist.selected() {
                            if id != index {}
                        }
                    }
                    None => todo!(),
                }
            }
        }
        _ => {}
    }
}
