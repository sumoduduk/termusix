use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear, Paragraph, StatefulWidget, Widget},
};

use tui_input::Input;

use crate::{
    app::app_state::AddSongState,
    user_interface::cursor::{AppState, Cursor},
};

use super::{centered_rect, list_folder_add::render_list_add_path};

#[derive(Debug, Default)]
pub struct AddMusicPopUp {
    pub input_path: Input,
    pub add_folder_song: AddSongState,
}

use Constraint::*;

impl AddMusicPopUp {
    fn input_file_path_song(&self, area: Rect, buf: &mut Buffer, cursor: &mut Cursor) {
        let block = Block::bordered().title("Insert Playlist");

        let width = area.width.max(3) - 3;
        let scroll = self.input_path.visual_scroll(width as usize);

        Paragraph::new(self.input_path.value())
            .scroll((0, scroll as u16))
            .block(block)
            .render(area, buf);

        cursor.set(
            area.x + ((self.input_path.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            area.y + 1,
        )
    }
}

impl StatefulWidget for &mut AddMusicPopUp {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let pop_up_area = centered_rect(60, 40, area);

        let pop_up_layout = Layout::vertical([Min(3), Percentage(100)]);
        let [input_layout, folder_layout] = pop_up_layout.areas(pop_up_area);

        let main_folder_layout = Layout::horizontal([Percentage(50), Percentage(50)]);
        let [add_folder_layout, add_song_layout] = main_folder_layout.areas(folder_layout);

        Clear.render(pop_up_area, buf);

        let folder_file = self.add_folder_song.serve_folder_file();
        let add_songs = self.add_folder_song.serve_add_paths();

        render_list_add_path(
            folder_file,
            add_folder_layout,
            buf,
            &mut state.add_song_state.file_scroll,
        );

        render_list_add_path(
            add_songs,
            add_song_layout,
            buf,
            &mut state.add_song_state.music_scroll,
        );

        self.input_file_path_song(input_layout, buf, &mut state.cursor);
    }
}
