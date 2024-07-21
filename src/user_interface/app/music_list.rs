use ratatui::widgets::ListState;

use crate::utils::shuffle_vec;

#[derive(Debug)]
pub struct MusicList {
    pub list_state: ListState,
    musics: Vec<String>,
}

impl Default for MusicList {
    fn default() -> Self {
        let list = [].to_vec();
        MusicList {
            list_state: ListState::default(),
            musics: list,
        }
    }
}

impl MusicList {
    pub fn get_list(&self) -> Vec<String> {
        self.musics.to_owned()
    }

    pub fn shuffle(&mut self) {
        shuffle_vec(&mut self.musics)
    }

    pub fn append_music(&mut self, list: Vec<String>) {
        self.musics = list;
    }
}
