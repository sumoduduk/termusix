use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, Padding, StatefulWidget},
};

use crate::app::{screen::Screen, App};

use super::SELECTED_STYLE;

pub fn render_music_list(app: &mut App, music_list_layout: Rect, buf: &mut Buffer) {
    let music_block = Block::new()
        .title("Music List")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, 1, 1))
        .border_style(app.get_border_color(Screen::ListMusic));

    let indx = app.tabs_playlist.selected();
    let music = app.list_downloaded_first(indx);

    let music_list = List::new(music)
        .block(music_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(music_list, music_list_layout, buf, &mut app.music_list);
}
