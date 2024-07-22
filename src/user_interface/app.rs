use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{ListState, Widget},
};
use screen::{get_border_color, Screen};
use std::sync::mpsc::Sender;

use crate::{file_ops::check_file_exist_sync, playback::PlaybackEvent, playlist::Playlist};

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
    pub music_list: ListState,
    pub screen_state: Screen,
    pub playlist: Playlist,
    pub tabs_playlist: ListState,
    pub tx_playback: Sender<PlaybackEvent>,
}

// impl Default for App {
//     fn default() -> Self {}
// }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tx: Sender<PlaybackEvent>) -> Self {
        let playlist = Playlist::new().expect("ERROR: No playlist found");

        Self {
            running: true,
            music_list: ListState::default(),
            screen_state: Screen::default(),
            playlist,
            tabs_playlist: ListState::default(),
            tx_playback: tx,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        let _ = self.tx_playback.send(PlaybackEvent::Quit);
        self.running = false;
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

    pub fn list_playlist_music(&self, indx: Option<usize>) -> Option<Vec<String>> {
        let music_l = self.playlist.list_music_by_idx(indx);
        if let Some(list) = music_l {
            let s: Vec<_> = list.into_values().collect();
            Some(s)
        } else {
            None
        }
    }

    pub fn list_downloaded_first(&self, indx: Option<usize>) -> Vec<String> {
        let Some(mut list_music) = self.playlist.list_music_by_idx(indx) else {
            return vec![];
        };

        let mut i = 0;
        for (id, _) in list_music.clone().iter() {
            let file_name = ["music/", id, ".mp3"].concat();
            if check_file_exist_sync(&file_name).is_some() {
                if let Some(index) = list_music.get_index_of(id) {
                    list_music.swap_indices(index, i);
                    i += 1;
                }
            }
        }
        let res: Vec<String> = list_music
            .clone()
            .values()
            .map(|name| name.to_owned())
            .collect();
        res
    }

    pub fn play_music(&self) {
        let idx = self.tabs_playlist.selected().unwrap_or(0);

        if let Some(playlist) = self.playlist.list_music_by_idx(Some(idx)) {
            let data: Vec<String> = playlist
                .keys()
                .map(|id| ["music/", id, ".mp3"].concat())
                .collect();

            let _ = self.tx_playback.send(PlaybackEvent::Playlist(data));
        }
    }

    pub fn pause_toggle(&self) {
        let _ = self.tx_playback.send(PlaybackEvent::PauseToggle);
    }

    pub fn next_music(&self) {
        let _ = self.tx_playback.send(PlaybackEvent::Forward);
    }

    pub fn prev_music(&self) {
        let _ = self.tx_playback.send(PlaybackEvent::Backward);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ui::render(self, area, buf);
    }
}
