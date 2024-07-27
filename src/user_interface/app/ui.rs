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
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::user_interface::cursor::Cursor;

use super::{screen::Screen, App};
use Constraint::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer, cursor: &mut Cursor) {
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
            let pop_up_area = centered_rect(60, 10, area);

            Clear.render(pop_up_area, buf);
            let block = Block::bordered().title("Insert Playlist");

            let width = pop_up_area.width.max(3) - 3;
            let scroll = app.input_playlist.visual_scroll(width as usize);

            Paragraph::new(app.input_playlist.value())
                .scroll((0, scroll as u16))
                .block(block)
                .render(pop_up_area, buf);

            cursor.set(
                pop_up_area.x
                    + ((app.input_playlist.visual_cursor()).max(scroll) - scroll) as u16
                    + 1,
                pop_up_area.y + 1,
            )
        }
        _ => {}
    }
}
