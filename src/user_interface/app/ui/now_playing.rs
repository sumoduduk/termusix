use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub fn display_now_playing(song_name: Option<String>, block: Block, area: Rect, buf: &mut Buffer) {
    let line_header = Line::from("Now Playing : ");
    let song_name = song_name.unwrap_or_default();

    let text = Text::from(vec![line_header, song_name.into()]).alignment(Alignment::Center);

    Paragraph::new(text)
        .block(block)
        .centered()
        .render(area, buf);
}
