use ratatui::{
    style::Color,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::App;

pub fn render_list_to_add(app: &App) -> Paragraph {
    let list_line: Vec<_> = app
        .list_to_add
        .iter()
        .filter_map(|p| p.file_name().and_then(|s| s.to_str().map(Line::from)))
        .collect();

    let text_line = Text::from(list_line);

    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Red);

    Paragraph::new(text_line).block(block)
}

pub fn render_pop_up_top() -> Paragraph<'static> {
    let msg = "↑ or ↓ to scroll | ← to go back folder | → to enter folder directory ";

    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Press ESC to Cancel | SPACE to add song | ENTER to Confirm")
        .border_style(Color::Red);

    Paragraph::new(msg).block(block).centered()
}
