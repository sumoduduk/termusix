use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListState, StatefulWidget},
};

use crate::app::ui::SELECTED_STYLE;

pub fn render_list_add_path(
    list_path: Vec<&str>,
    rect: Rect,
    buf: &mut Buffer,
    scroll: &mut ListState,
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Press Q to quit app")
        .border_style(Color::White);
    List::new(list_path)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .render(rect, buf, scroll);
}
