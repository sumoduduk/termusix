mod file_ops;
mod file_song;
mod playback;
mod playlist;
mod user_interface;
mod utils;

use app::{screen::Screen, App};
use handler::handle_key_events;
use handler::input_playlist_handler::handle_key_input_events;
use playback::start_playing;
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use std::env::args;
use std::io;
use std::sync::{Arc, RwLock};
use utils::convert::convert_folder;

use user_interface::event::Event;
use user_interface::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoResult {
    title: String,
    video_id: String,
    author: String,
    length_seconds: usize,
}

#[derive(Debug, Default)]
struct NowPlayingData {
    playlist_id: Option<usize>,
    song_title: Option<String>,
}

type NowPlaying = Arc<RwLock<NowPlayingData>>;

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
    let mut args = args();
    args.next();

    if let Some(arg) = args.next() {
        match arg.trim() {
            "--convert" => {
                convert_folder().await?;
            }

            _ => {
                println!("INFO: use argument --convert to convert file folder");
            }
        }
    } else {
        // Create an application.

        let (tx, rx) = std::sync::mpsc::channel();
        let now_playing: NowPlaying = Arc::new(RwLock::new(NowPlayingData::default()));
        let now_playing_app = Arc::clone(&now_playing);

        let mut app = App::new(tx, now_playing_app);
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
                Event::Key(key_event) => match app.screen_state {
                    Screen::InsertPlaylist => handle_key_input_events(key_event, &mut app).await?,

                    _ => {
                        handle_key_events(key_event, &mut app).await?;
                    }
                },
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
        }

        // Exit the user interface.
        tui.exit()?;
    }
    Ok(())
}
