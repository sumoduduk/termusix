mod handle_d;
mod handle_enter;
mod handle_up_down;
pub mod input_playlist_handler;
mod insert_playlist_song;
mod play;

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use handle_d::handle_delete_key;
use handle_enter::enter_key;
use handle_up_down::{handle_key_down, handle_key_up};
use insert_playlist_song::handle_a;
use play::play_and_download;

use super::app::screen::Screen;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Esc => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Tab => {
            app.next_screen();
        }

        KeyCode::Up => handle_key_up(app),
        KeyCode::Down => handle_key_down(app),
        KeyCode::Enter => enter_key(app).await,
        KeyCode::Char('k') => handle_key_up(app),
        KeyCode::Char('j') => handle_key_down(app),
        KeyCode::Char('a') => handle_a(app),
        KeyCode::Char('d') => handle_delete_key(app),
        KeyCode::Char('p') => match app.screen_state {
            Screen::Playlist => {
                play_and_download(app).await;
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

        //add another
        _ => {}
    }
    Ok(())
}
