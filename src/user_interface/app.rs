use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{ListState, StatefulWidget},
};
use screen::{get_border_color, Screen};
use std::sync::mpsc::Sender;
use tui_input::Input;
use ui::add_music_widget::AddMusicPopUp;

use crate::{
    file_song::{FileExplorer, Theme},
    playback::PlaybackEvent,
    playlist::Playlist,
    NowPlaying,
};

use super::cursor::AppState;

mod app_state;
pub mod screen;
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
    pub now_playing: NowPlaying,
    pub input_playlist: Input,
    pub add_song_popup: AddMusicPopUp,
    pub file_explorer: FileExplorer,
}

// impl Default for App {
//     fn default() -> Self {}
// }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tx: Sender<PlaybackEvent>, now_playing: NowPlaying) -> Self {
        let playlist = Playlist::new().expect("ERROR: No playlist found");

        let theme = Theme::default().add_default_title();
        let file_explorer = FileExplorer::with_theme(theme).unwrap();

        Self {
            running: true,
            music_list: ListState::default(),
            screen_state: Screen::default(),
            playlist,
            tabs_playlist: ListState::default(),
            tx_playback: tx,
            now_playing,
            input_playlist: Input::default(),
            add_song_popup: AddMusicPopUp::default(),
            file_explorer,
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
            _ => {}
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
        let Some((list_music, _)) = self.playlist.list_music_sorted(indx) else {
            return vec![];
        };

        let res: Vec<String> = list_music.values().map(|name| name.to_owned()).collect();
        res
    }

    pub fn list_id_downloaded_first(&self, indx: Option<usize>) -> Option<(Vec<String>, usize)> {
        let (list_music, i) = self.playlist.list_music_sorted(indx)?;

        let res: Vec<String> = list_music.keys().map(|name| name.to_owned()).collect();
        Some((res, i))
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

    pub fn get_now_playing(&self) -> Option<String> {
        let id = if let Ok(ids) = self.now_playing.read() {
            let name_id = ids.as_ref()?;

            let selected = self.tabs_playlist.selected();
            let list = self.playlist.list_music_sorted(selected)?;
            let name_song = list.0.get_key_value(name_id)?;
            Some(name_song.1.to_owned())
        } else {
            None
        };
        id
    }
}

impl StatefulWidget for &mut App {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ui::render(self, area, buf, state);
    }
}
