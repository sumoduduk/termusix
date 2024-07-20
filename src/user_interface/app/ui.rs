use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{self, SLATE},
        Color, Modifier, Style,
    },
    symbols::block,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, Paragraph, StatefulWidget, Tabs, Widget,
    },
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

    let musics = app.music_list.get_list();

    let music_list = List::new(musics)
        .block(music_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    playback_block.render(playback_layout, buf);
    // playlist_block.render(playlist_layout, buf);

    StatefulWidget::render(
        music_list,
        music_list_layout,
        buf,
        &mut app.music_list.list_state,
    );

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

fn render_tab(app: &App, area: Rect, buf: &mut Buffer, block: Block) {
    let titles = app.playlist.list_playlist_titles();
    let hg_style = (Color::default(), tailwind::BLUE.c500);

    let selected = app.tabs_playlist;

    Tabs::new(titles)
        .select(selected)
        .highlight_style(hg_style)
        .padding("", "")
        .divider(" ")
        .block(block)
        .render(area, buf);
}
