mod music_list_components;
mod playback_component;
mod playlist_components;

use music_list_components::render_music_list;
use playback_component::render_playback;
use playlist_components::render_playlist;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
};

use super::App;
use Constraint::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let horizontal_screen = Layout::horizontal([Percentage(35), Percentage(65)]);

    let [main_layout, music_list_layout] = horizontal_screen.areas(area);
    let main_frame = Layout::vertical([Percentage(50), Percentage(50)]);
    let [playback_layout, playlist_layout] = main_frame.areas(main_layout);

    let song_name = app.get_now_playing();

    render_playback(app, playback_layout, buf, song_name);
    render_playlist(app, playlist_layout, buf);
    render_music_list(app, music_list_layout, buf)
}
