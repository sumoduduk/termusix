use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{palette::tailwind, Color, Stylize},
    symbols::block,
    text::Line,
    widgets::{Block, Padding, Tabs, Widget},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedButton {
    #[strum(to_string = "|◄◄")]
    Prevous,
    #[strum(to_string = "◄◄")]
    Rewind,
    #[default]
    #[strum(to_string = " ▶ ")]
    Play,
    #[strum(to_string = "►►")]
    Forward,
    #[strum(to_string = "►►|")]
    Next,
}

impl SelectedButton {
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(tailwind::WHITE).into()
    }
}

impl Widget for SelectedButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let centered = centered_rect(80, 100, area);
        let titles = SelectedButton::iter().map(SelectedButton::title);
        let highlight_style = (Color::default(), tailwind::ROSE.c700);
        let selected_tab_index = self as usize;
        let block = Block::bordered()
            .border_style(Color::Red)
            .padding(Padding::new(3, 3, 0, 0));

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" | ")
            .block(block)
            .render(area, buf);
    }
}
