use ratatui::style::{
    palette::material::{RED, WHITE},
    Style,
};

const RED_BORDER: Style = Style::new().fg(RED.c400);
const WHITE_BORDER: Style = Style::new().fg(WHITE);

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Screen {
    Playback,
    Playlist,
    #[default]
    ListMusic,
}

pub fn get_border_color(app_screen: &Screen, screen: Screen) -> Style {
    if *app_screen == screen {
        RED_BORDER
    } else {
        WHITE_BORDER
    }
}
