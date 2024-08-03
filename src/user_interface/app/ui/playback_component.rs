use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    text::Text,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::app::{App, Screen};

pub fn render_playback(app: &App, area: Rect, buf: &mut Buffer) {
    let title_right = app.render_title_right(Screen::Playback).unwrap_or_default();
    let playback_block = Block::new()
        .title("Now Playing")
        .title(Title::from(title_right).alignment(Alignment::Right))
        .borders(Borders::ALL)
        .padding(Padding::new(0, 0, area.height / 4, 0))
        .border_type(BorderType::Rounded)
        .border_style(app.get_border_color(Screen::Playback));

    let inner_area = playback_block.inner(area);

    let playback_area =
        Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)]).flex(Flex::Center);

    let [playback_layout, button_layout] = playback_area.areas(inner_area);

    let song_name = app.get_now_playing();

    playback_block.render(area, buf);
    display_now_playing(song_name, playback_layout, buf);
    app.playback_button.render(button_layout, buf);
}

fn display_now_playing(song_name: Option<String>, area: Rect, buf: &mut Buffer) {
    let song_name = song_name.unwrap_or_default();

    let block = Block::new().padding(Padding {
        left: 2,
        right: 2,
        top: 0,
        bottom: 0,
    });

    let text = Text::from(song_name).alignment(Alignment::Center);

    Paragraph::new(text)
        .centered()
        .block(block)
        .wrap(Wrap { trim: true })
        .render(area, buf);
}
