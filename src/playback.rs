use std::{collections::VecDeque, fs::File};

use rodio::{OutputStream, Sink};
use std::sync::mpsc::Receiver;

pub enum PlaybackEvent {
    Playlist(Vec<String>),
    PauseToggle,
    Add(String),
    Forward,
    Backward,
    Quit,
}

pub fn start_playing(rx: Receiver<PlaybackEvent>) {
    tokio::task::spawn_blocking(move || {
        let (_stream, stram_handle) =
            OutputStream::try_default().expect("ERROR: error getting OutputStream");
        let sink = Sink::try_new(&stram_handle).expect("ERROR: play new sink");

        let mut playlist = VecDeque::new();
        let mut is_played = true;

        while is_played {
            if let Ok(evt) = rx.try_recv() {
                match evt {
                    PlaybackEvent::Playlist(list) => {
                        playlist = VecDeque::from(list);
                        sink.clear();
                    }
                    PlaybackEvent::PauseToggle => {
                        if sink.is_paused() {
                            sink.play();
                        } else {
                            sink.pause();
                        }
                    }
                    PlaybackEvent::Add(id) => {
                        playlist.push_back(id);
                    }
                    PlaybackEvent::Quit => {
                        sink.clear();
                        is_played = false;
                    }
                    PlaybackEvent::Forward => {
                        sink.clear();
                    }
                    PlaybackEvent::Backward => {
                        if !playlist.is_empty() {
                            if let Some(id) = playlist.pop_back() {
                                playlist.push_front(id);

                                if let Some(id_2) = playlist.pop_back() {
                                    playlist.push_front(id_2);
                                }

                                sink.clear();
                            }
                        }
                    } // _ => {}
                }
            }

            if sink.empty() {
                if let Some(music_file) = playlist.pop_front() {
                    if let Ok(file) = File::open(&music_file) {
                        match rodio::Decoder::new(file) {
                            Ok(source) => {
                                sink.append(source);
                                sink.play();
                            }
                            Err(err) => {
                                eprintln!("ERROR: can't add {} into playlist {err}", &music_file)
                            }
                        }
                        playlist.push_back(music_file);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(900));
        }
    });
}
