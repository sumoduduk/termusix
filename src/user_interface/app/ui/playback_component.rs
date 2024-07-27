use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::app::{App, Screen};

pub fn render_playback(app: &App, playback_layout: Rect, buf: &mut Buffer) {
    let playback_block = Block::new()
        .title("Now Playing")
        .borders(Borders::ALL)
        .padding(Padding::new(0, 0, playback_layout.height / 4, 0))
        .border_type(BorderType::Rounded)
        .border_style(app.get_border_color(Screen::Playback));

    let song_name = app.get_now_playing();

    display_now_playing(song_name, playback_block, playback_layout, buf)
}

fn display_now_playing(song_name: Option<String>, block: Block, area: Rect, buf: &mut Buffer) {
    let song_name = song_name.unwrap_or_default();

    let text = Text::from(song_name).alignment(Alignment::Center);

    Paragraph::new(text)
        .block(block)
        .centered()
        .render(area, buf);
}
