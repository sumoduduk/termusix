use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::{App, Screen};

pub fn render_footer(app: &App, footer_layout: Rect, buf: &mut Buffer) {
    let footer_msg = match app.screen_state {
        Screen::Playback => "Press TAB to switch to Playlist | SPACE to toggle play/pause",
        Screen::Playlist => {
            "Press TAB/ENTER to switch to Music List | ⬆️ or ⬇️ to scroll playlist | P to play"
        }
        Screen::ListMusic => {
            "Press TAB to switch to Now Playing | ⬆️ or ⬇️ to scroll song | ENTER to play "
        }
        _ => "",
    };

    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Press Q to quit app")
        .border_style(Color::Red);

    Paragraph::new(footer_msg)
        .block(block)
        .centered()
        .render(footer_layout, buf);
}
