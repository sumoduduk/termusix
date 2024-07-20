use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct MusicList {
    pub list_state: ListState,
    musics: Vec<String>,
}

impl Default for MusicList {
    fn default() -> Self {
        let list = ["Laruku", "KingGnU", "Popolocrois", "Aot"];
        MusicList {
            list_state: ListState::default(),
            musics: list.into_iter().map(|m| m.to_owned()).collect(),
        }
    }
}

impl MusicList {
    pub fn get_list(&self) -> Vec<String> {
        self.musics.to_owned()
    }
}
