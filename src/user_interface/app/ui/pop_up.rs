pub mod add_music_widget;
mod list_folder_add;
mod pop_up_plylist;

use pop_up_plylist::render_popup_playlist;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::StatefulWidget,
};

use crate::{
    app::{screen::Screen, App},
    user_interface::cursor::AppState,
};

pub fn render_popup(app: &mut App, area: Rect, buf: &mut Buffer, app_state: &mut AppState) {
    match &app.screen_state {
        Screen::InsertPlaylist => render_popup_playlist(app, area, buf, &mut app_state.cursor),
        Screen::AddSongPopup(_add_screen) => app.add_song_popup.render(area, buf, app_state),
        _ => {}
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
