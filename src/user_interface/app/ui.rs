use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    // style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use super::App;

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    use Constraint::*;
    let horizontal = Layout::horizontal([Percentage(35), Percentage(65)]);
    let [main_layout, playlist_layout] = horizontal.areas(area);

    let main_block = Block::new().title("Main").borders(Borders::ALL);
    let playlist_block = Block::new().title("Playlist").borders(Borders::ALL);

    main_block.render(main_layout, buf);
    playlist_block.render(playlist_layout, buf);

    // Paragraph::new(format!(
    //     "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //     app.counter
    // ))
    // .block(
    //     Block::bordered()
    //         .title("Template")
    //         .title_alignment(Alignment::Center)
    //         .border_type(BorderType::Rounded),
    // )
    // .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    // .centered()
    // .render(area, buf);
}
