mod add_music_widget;
mod list_folder_add;
mod pop_up_plylist;

use pop_up_plylist::render_popup_playlist;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    app::{screen::Screen, App},
    user_interface::cursor::Cursor,
};

pub fn render_popup(app: &App, area: Rect, buf: &mut Buffer, cursor: &mut Cursor) {
    match app.screen_state {
        Screen::InsertPlaylist => render_popup_playlist(app, area, buf, cursor),
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
