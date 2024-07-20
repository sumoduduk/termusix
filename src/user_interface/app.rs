use music_list::MusicList;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{ListState, Widget},
};
use screen::{get_border_color, Screen};

use crate::playlist::Playlist;

mod music_list;
pub mod screen;
mod tabs_playlist;
mod ui;
// use std::error;

/// Application result type.
// pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type AppResult<T> = eyre::Result<T>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub counter: u8,
    pub music_list: MusicList,
    pub screen_state: Screen,
    pub playlist: Playlist,
    pub tabs_playlist: ListState,
}

impl Default for App {
    fn default() -> Self {
        let playlist = Playlist::new().expect("ERROR: No playlist found");

        Self {
            running: true,
            counter: 0,
            music_list: MusicList::default(),
            screen_state: Screen::default(),
            playlist,
            tabs_playlist: ListState::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    // pub fn next_tab(&mut self) {
    //     let len = self.playlist.list_playlist_titles().len();
    //     let i = self.tabs_playlist;
    //
    //     if i >= len {
    //         self.tabs_playlist = 0;
    //     } else {
    //         self.tabs_playlist += 1;
    //     }
    // }
    //
    // pub fn prev_tab(&mut self) {
    //     let len = self.playlist.list_playlist_titles().len();
    //     let i = self.tabs_playlist;
    //
    //     if i == 0 {
    //         self.tabs_playlist = len - 1;
    //     } else {
    //         self.tabs_playlist -= 1;
    //     }
    // }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn next_screen(&mut self) {
        use Screen::*;

        match self.screen_state {
            Playback => self.screen_state = Playlist,
            Playlist => self.screen_state = ListMusic,
            ListMusic => self.screen_state = Playback,
        };
    }

    pub fn get_border_color(&self, screen: Screen) -> Style {
        get_border_color(&self.screen_state, screen)
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ui::render(self, area, buf);
    }
}
