mod download;
mod file_ops;
mod playback;
mod playlist;
mod user_interface;
mod utils;

use handler::handle_key_events;
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::{Arc, RwLock};

use user_interface::event::Event;
use user_interface::*;

type NowPlaying = Arc<RwLock<Option<String>>>;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoResult {
    title: String,
    video_id: String,
    author: String,
    length_seconds: usize,
}

#[derive(Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    title: String,
    playlist_id: String,
    author: String,
    video_count: usize,
    videos: Vec<VideoResult>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create an application.

    let (tx, rx) = std::sync::mpsc::channel();
    let now_playing: NowPlaying = Arc::new(RwLock::new(None::<String>));
    let now_playing_app = Arc::clone(&now_playing);

    let mut app = app::App::new(tx, now_playing_app);
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = event::EventHandler::new(250);
    let mut tui = tui::Tui::new(terminal, events);
    tui.init()?;
    start_playing(rx, now_playing);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;

    Ok(())
}
