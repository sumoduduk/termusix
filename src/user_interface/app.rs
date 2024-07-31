use cwd_dirs::get_music_path;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{ListState, StatefulWidget},
};
use screen::{get_border_color, Screen};
use std::{path::PathBuf, sync::mpsc::Sender};
use tui_input::Input;

use crate::{
    file_song::{FileExplorer, Theme},
    playback::PlaybackEvent,
    playlist::Playlist,
    NowPlaying,
};

use super::cursor::AppState;

mod cwd_dirs;
pub mod screen;
mod ui;
pub mod widget_list_add;
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
    pub file_explorer: FileExplorer,
    pub list_to_add: Vec<PathBuf>,
}

// impl Default for App {
//     fn default() -> Self {}
// }

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tx: Sender<PlaybackEvent>, now_playing: NowPlaying) -> Self {
        let playlist = Playlist::new().expect("ERROR: No playlist found");

        let theme = Theme::default().add_default_title();
        let mut file_explorer = FileExplorer::with_theme(theme).unwrap();

        if let Some(music_path) = get_music_path() {
            let _ = file_explorer.set_cwd(music_path);
        }

        Self {
            running: true,
            music_list: ListState::default(),
            screen_state: Screen::default(),
            playlist,
            tabs_playlist: ListState::default(),
            tx_playback: tx,
            now_playing,
            input_playlist: Input::default(),
            file_explorer,
            list_to_add: Vec::default(),
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

    pub fn push_song_from_explorer(&mut self) {
        let path = self.file_explorer.current().path();
        let extention = path.extension().and_then(|os_str| os_str.to_str());

        if let Some(ext) = extention {
            if ext == "mp4" || ext == "mp3" {
                self.list_to_add.push(path.into())
            }
        }
    }

    pub fn save_song_to_playlist(&mut self) {
        let paths_songs = &self.list_to_add;

        if !paths_songs.is_empty() {
            if let Some(index) = self.tabs_playlist.selected() {
                if self.playlist.save_local_song(paths_songs, index).is_ok() {
                    self.list_to_add.clear();
                }
            }
        }
    }
}

impl StatefulWidget for &mut App {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        ui::render(self, area, buf, state);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
