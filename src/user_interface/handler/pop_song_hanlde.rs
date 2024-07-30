use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{
    screen::{AddSongPopup, Screen},
    App, AppResult,
};

pub async fn handle_pop_sog_events(
    key_event: KeyEvent,
    app: &mut App,
    screen_popup: &AddSongPopup,
) -> AppResult<()> {
    match screen_popup {
        AddSongPopup::InsertFolder => match key_event.code {
            KeyCode::Esc => app.screen_state = Screen::ListMusic,
            _ => {
                app.add_song_popup
                    .input_path
                    .handle_event(&Event::Key(key_event));
            }
        },
        AddSongPopup::FolderPath => todo!(),
        AddSongPopup::AddSong => todo!(),
    }

    Ok(())
}
