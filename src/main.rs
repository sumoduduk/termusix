mod file_ops;
mod music_func;
mod playlist;
mod user_interface;
mod utils;

use handler::handle_key_events;
use music_func::play;
use playlist::Playlist;
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use std::env::args;
use std::io;

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

    let arg = args
        .next()
        .expect("ERROR: argument expected => --save, --list, --play");

    dbg!(&arg);

    let mut playlist = Playlist::new()?;

    match arg.trim() {
        "--save" => {
            let url = args.next().expect("ERROR: provide the URL");
            playlist.save_playlist(&url).await?;
            println!("Playlist saved..")
        }
        "--save-id" => {
            let id = args.next().expect("ERROR: provide the ID");
            playlist.save_by_id(&id).await?;
            println!("Playlist ID saved..")
        }
        "--list" => {
            playlist.list_playlist();
        }

        "--play" => {
            let id = args.next().expect("ERROR: provide the ID");
            let list_id = playlist.list_shuffled_music_id(&id);

            match list_id {
                Ok(list) => play(list).await?,
                Err(error) => println!("{error}"),
            }
        }

        "--ui" => {
            // Create an application.
            let mut app = app::App::new();

            // Initialize the terminal user interface.
            let backend = CrosstermBackend::new(io::stderr());
            let terminal = Terminal::new(backend)?;
            let events = event::EventHandler::new(250);
            let mut tui = tui::Tui::new(terminal, events);
            tui.init();

            // Start the main loop.
            while app.running {
                // Render the user interface.
                tui.draw(&mut app)?;
                // Handle events.
                match tui.events.next() {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }

            // Exit the user interface.
            tui.exit()?;
        }

        _ => println!("Please provide correct argument, --save, --list, --play"),
    }

    Ok(())
}
