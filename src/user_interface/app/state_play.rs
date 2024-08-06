use ratatui::{
    style::{Color, Stylize},
    text::Line,
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum StatePlay {
    #[default]
    #[strum(to_string = "Normal")]
    Normal,
    #[strum(to_string = "Repeat One")]
    RepeatOne,
    #[strum(to_string = "Random")]
    Random,
}

impl StatePlay {
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(Self::Normal)
    }

    pub fn title(self) -> Line<'static> {
        format!("Mode : {self}").fg(Color::White).into()
    }
}
