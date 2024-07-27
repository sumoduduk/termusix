mod footer_components;
mod music_list_components;
mod playback_component;
mod playlist_components;
mod pop_up;

use footer_components::render_footer;
use music_list_components::render_music_list;
use playback_component::render_playback;
use playlist_components::render_playlist;
use pop_up::centered_rect;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{Block, Clear, Widget},
};

use super::{screen::Screen, App};
use Constraint::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let main_screen = Layout::vertical([Percentage(100), Min(3)]);
    let [main_layout, footer_layout] = main_screen.areas(area);

    let horizontal_screen = Layout::horizontal([Percentage(35), Percentage(65)]);
    let [main_layout, music_list_layout] = horizontal_screen.areas(main_layout);

    let main_frame = Layout::vertical([Percentage(50), Percentage(50)]);
    let [playback_layout, playlist_layout] = main_frame.areas(main_layout);

    render_playback(app, playback_layout, buf);
    render_playlist(app, playlist_layout, buf);
    render_music_list(app, music_list_layout, buf);
    render_footer(app, footer_layout, buf);

    #[allow(clippy::single_match)]
    match app.screen_state {
        Screen::InsertPlaylist => {
            let pop_up_area = centered_rect(60, 20, area);

            Clear.render(pop_up_area, buf);
            Block::bordered()
                .title("Insert Playlist")
                .render(pop_up_area, buf);
        }
        _ => {}
    }
}
