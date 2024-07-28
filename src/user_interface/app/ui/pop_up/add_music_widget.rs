use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Clear, StatefulWidget, Widget},
};
use tui_input::Input;

use crate::app::app_state::AddSongState;

use super::{centered_rect, list_folder_add::render_list_add_path};

#[derive(Default)]
struct AddMusicPopUp {
    input_path: Input,
}

use Constraint::*;

impl StatefulWidget for AddMusicPopUp {
    type State = AddSongState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let pop_up_area = centered_rect(60, 40, area);

        let pop_up_layout = Layout::vertical([Min(2), Percentage(100)]);
        let [input_layout, folder_layout] = pop_up_layout.areas(pop_up_area);

        let main_folder_layout = Layout::horizontal([Percentage(50), Percentage(50)]);
        let [add_folder_layout, add_song_layout] = main_folder_layout.areas(folder_layout);

        Clear.render(pop_up_area, buf);

        let folder_file = state.serve_folder_file();
        let add_songs = state.serve_add_paths();

        render_list_add_path(folder_file, add_folder_layout, buf, &mut state.file_scroll);
    }
}
