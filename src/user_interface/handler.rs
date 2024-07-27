mod handle_enter;
mod handle_up_down;
mod play;

use crate::app::{App, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use handle_enter::enter_key;
use handle_up_down::{handle_key_down, handle_key_up};
use play::play_and_download;

use super::app::screen::Screen;
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Esc => match app.screen_state {
            Screen::InsertPlaylist => app.screen_state = Screen::ListMusic,
            _ => app.quit(),
        },
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
        KeyCode::Char('k') => handle_key_up(app),
        KeyCode::Char('j') => handle_key_down(app),
        KeyCode::Enter => enter_key(app),
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

        KeyCode::Char('a') => {
            app.screen_state = Screen::InsertPlaylist;
        }

        //add another
        _ => match app.screen_state {
            Screen::InsertPlaylist => {
                app.input_playlist.handle_event(&Event::Key(key_event));
            }

            _ => {}
        },
    }
    Ok(())
}
