use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, StatefulWidget, Widget},
};

use super::App;
use super::Screen::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    use Constraint::*;
    let horizontal_screen = Layout::horizontal([Percentage(35), Percentage(65)]);
    let [main_layout, music_list_layout] = horizontal_screen.areas(area);
    let main_frame = Layout::vertical([Percentage(50), Percentage(50)]);
    let [playback_layout, playlist_layout] = main_frame.areas(main_layout);

    let playback_block = Block::new()
        .title("Playback")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(app.get_border_color(Playback));

    let playlist_block = Block::new()
        .title("Playlist")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(app.get_border_color(Playlist));

    let music_block = Block::new()
        .title("Music List")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(app.get_border_color(ListMusic));

    let indx = app.tabs_playlist.selected();
    let music = app.list_downloaded_first(indx);

    let music_list = List::new(music)
        .block(music_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    playback_block.render(playback_layout, buf);
    // playlist_block.render(playlist_layout, buf);

    StatefulWidget::render(music_list, music_list_layout, buf, &mut app.music_list);

    render_tab(app, playlist_layout, buf, playlist_block);

    // Paragraph::new(format!(
    //     "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //     app.counter
    // ))
    // .block(
    //     Block::bordered()
    //         .title("Template")
    //         .title_alignment(Alignment::Center)
    //         .border_type(BorderType::Rounded),
    // )
    // .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    // .centered()
    // .render(area, buf);
}

fn render_tab(app: &mut App, area: Rect, buf: &mut Buffer, block: Block) {
    // let hg_style = (Color::default(), tailwind::BLUE.c500);
    let titles = app.playlist.list_playlist_titles();

    let list = List::new(titles)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, buf, &mut app.tabs_playlist);
}
