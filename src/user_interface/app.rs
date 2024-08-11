use cwd_dirs::get_music_path;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{ListState, StatefulWidget},
};
use rodio::Sample;
use screen::{get_border_color, Screen};
use state_play::StatePlay;
use std::{path::PathBuf, sync::mpsc::Sender};
use tui_input::Input;
use widget_playback_buttons::SelectedButton;

use crate::{
    file_song::{FileExplorer, Theme},
    playback::PlaybackEvent,
    playlist::Playlist,
    NowPlaying,
};

use super::cursor::AppState;

mod cwd_dirs;
pub mod screen;
pub mod state_play;
mod ui;
pub mod widget_list_add;
pub mod widget_playback_buttons;
// use std::error;

/// Application result type.
// pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type AppResult<T> = eyre::Result<T>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub state_play: StatePlay,
    pub music_list: ListState,
    pub screen_state: Screen,
    pub playlist: Playlist,
    pub tabs_playlist: ListState,
    pub tx_playback: Sender<PlaybackEvent>,
    pub now_playing: NowPlaying,
    pub input_playlist: Input,
    pub file_explorer: FileExplorer,
    pub list_to_add: Vec<PathBuf>,
    pub pop_up_confirm: bool,
    pub playback_button: SelectedButton,
    pub volume: f32,
    pub is_mute: bool,
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
            state_play: StatePlay::Normal,
            music_list: ListState::default(),
            screen_state: Screen::default(),
            playlist,
            tabs_playlist: ListState::default(),
            tx_playback: tx,
            now_playing,
            input_playlist: Input::default(),
            file_explorer,
            list_to_add: Vec::default(),
            pop_up_confirm: false,
            playback_button: SelectedButton::default(),
            volume: 1.0,
            is_mute: false,
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
        let music_l = self.playlist.list_music_by_idx(indx)?;

        let list: Vec<_> = music_l.values().map(|s| s.to_owned()).collect();
        Some(list)
    }

    pub fn list_playlist_song_id(&self, indx: Option<usize>) -> Option<Vec<String>> {
        let music_l = self.playlist.list_music_by_idx(indx)?;

        let list: Vec<_> = music_l.keys().map(|s| s.to_owned()).collect();
        Some(list)
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
            ids.song_title.clone()
        } else {
            None
        };
        id
    }

    pub fn push_song_from_explorer(&mut self) {
        let path = self.file_explorer.current().path();
        let extention = path.extension().and_then(|os_str| os_str.to_str());

        if let Some(ext) = extention {
            if ext == "flac" || ext == "mp3" || ext == "wav" || ext == "ogg" {
                self.list_to_add.push(path.into())
            }
        }
    }

    pub fn get_now_playing_id(&self) -> Option<usize> {
        if let Ok(now_playing) = self.now_playing.read() {
            now_playing.playlist_id
        } else {
            None
        }
    }

    pub fn save_song_to_playlist(&mut self) {
        let paths_songs = &self.list_to_add;

        if !paths_songs.is_empty() {
            if let Some(index) = self.tabs_playlist.selected() {
                if let Some(idx) = self.get_now_playing_id() {
                    if index == idx {
                        paths_songs.iter().for_each(|p| {
                            let sender = self.tx_playback.clone();
                            let _ = sender.send(PlaybackEvent::Add(p.to_path_buf()));
                        })
                    }
                }

                if self.playlist.save_local_song(paths_songs, index).is_ok() {
                    self.list_to_add.clear();
                }
            }
        }
    }

    pub fn pop_up_msg(&self) -> Option<(&str, String)> {
        let msg = "Are you sure you want to delete  ";

        let index_playlist = self.tabs_playlist.selected();

        match self.screen_state {
            Screen::Playlist => self
                .playlist
                .get_playlist_title(index_playlist)
                .map(|s| ("Delete Playlist", [msg, s, "?"].concat())),

            Screen::ListMusic => {
                let index_song = self.music_list.selected();
                self.playlist
                    .get_music_title(index_playlist, index_song)
                    .map(|s| ("Delete Song", [msg, s, "?"].concat()))
            }

            _ => None,
        }
    }

    pub fn button_next(&mut self) {
        self.playback_button = self.playback_button.next();
    }

    pub fn button_prev(&mut self) {
        self.playback_button = self.playback_button.previous();
    }

    pub fn mode_next(&mut self) {
        let state = self.state_play.next();
        self.state_play = state;

        let sender = self.tx_playback.clone();
        let _ = sender.send(PlaybackEvent::State(state));
    }

    fn render_title_right_inner(&self) -> Option<String> {
        match self.screen_state {
            Screen::Playback => Some("← → to scroll | + - for volume ".to_owned()),
            Screen::Playlist => Some("Press A to add playlist".to_owned()),
            Screen::ListMusic => Some("Press A to add song".to_owned()),
            _ => None,
        }
    }

    pub fn render_title_right(&self, screen: Screen) -> Option<String> {
        if self.screen_state == screen {
            self.render_title_right_inner()
        } else {
            None
        }
    }

    pub fn seek_forward(&self) {
        let sender = self.tx_playback.clone();
        let _ = sender.send(PlaybackEvent::SeekForward);
    }

    pub fn seek_backward(&self) {
        let sender = self.tx_playback.clone();
        let _ = sender.send(PlaybackEvent::SeekBackward);
    }

    pub fn increase_volume(&mut self) {
        let total = self.volume.saturating_add(0.1);

        if total > 2.0 {
            self.volume = 2.0;
        } else {
            self.volume = total;
        }
    }

    pub fn decrease_volume(&mut self) {
        let total = self.volume.saturating_add(-0.1);

        if total < 0.0 {
            self.volume = 0.0;
        } else {
            self.volume = total;
        }
    }

    pub fn mute_toggle(&self) {
        let sender = self.tx_playback.clone();
        let _ = sender.send(PlaybackEvent::Mute(self.volume));
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
