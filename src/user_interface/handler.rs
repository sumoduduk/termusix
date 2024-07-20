use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::screen::Screen;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }

        KeyCode::Tab => {
            app.next_screen();
        }

        KeyCode::Char('j') => match app.screen_state {
            Screen::Playback => {}
            Screen::Playlist => app.next_tab(),
            Screen::ListMusic => {
                app.music_list.list_state.select_next();
            }
        },
        KeyCode::Char('k') => match app.screen_state {
            Screen::Playback => {}
            Screen::Playlist => app.prev_tab(),
            Screen::ListMusic => {
                app.music_list.list_state.select_previous();
            }
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
