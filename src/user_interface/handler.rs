use crate::{
    app::{App, AppResult},
    download::download_music,
    playback::PlaybackEvent,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::screen::Screen;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        // KeyCode::Right => {
        //     app.increment_counter();
        // }
        KeyCode::Tab => {
            app.next_screen();
        }

        KeyCode::Char('j') => match app.screen_state {
            Screen::Playlist => {
                app.tabs_playlist.select_next();
            }
            Screen::ListMusic => {
                app.music_list.select_next();
            }

            _ => {}
        },
        KeyCode::Char('k') => match app.screen_state {
            Screen::Playlist => {
                app.tabs_playlist.select_previous();
            }
            Screen::ListMusic => {
                app.music_list.select_previous();
            }
            _ => {}
        },
        #[allow(clippy::single_match)]
        KeyCode::Char('p') => match app.screen_state {
            Screen::Playlist => {
                // app.play_music();

                let indx = app.tabs_playlist.selected();
                let send = app.tx_playback.clone();
                if let Some((list, len)) = app.list_id_downloaded_first(indx) {
                    tokio::spawn(async move {
                        let downloaded: Vec<String> = list[..len]
                            .to_vec()
                            .iter()
                            .map(|id| concat_file(id))
                            .collect();

                        if !downloaded.is_empty() {
                            let _ = send.send(PlaybackEvent::Playlist(downloaded));
                        }
                        let not_downloaded = &list[len..].to_vec();
                        for id in not_downloaded {
                            let res = download_music(id).await;
                            if res.is_ok() {
                                let _ = send.send(PlaybackEvent::Add(concat_file(id)));
                            }
                        }
                    });
                }
            }
            _ => {}
        },

        KeyCode::Char(' ') => {
            app.pause_toggle();
        }

        KeyCode::PageUp => {
            app.prev_music();
        }

        KeyCode::PageDown => {
            app.next_music();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

fn concat_file(id: &str) -> String {
    ["music", id, ".mp3"].concat()
}
