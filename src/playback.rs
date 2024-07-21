use std::{collections::VecDeque, fs::File};

use rodio::{OutputStream, Sink};
use tokio::sync::mpsc::Receiver;

pub enum PlaybackEvent {
    Playlist(Vec<String>),
    PauseToggle,
    Forward,
    Backward,
}

pub fn start_playing(mut rx: Receiver<PlaybackEvent>) {
    tokio::task::spawn_blocking(move || {
        let (_stream, stram_handle) =
            OutputStream::try_default().expect("ERROR: error getting OutputStream");
        let sink = Sink::try_new(&stram_handle).expect("ERROR: play new sink");

        let mut playlist = VecDeque::new();

        while let Some(evt) = rx.blocking_recv() {
            match evt {
                PlaybackEvent::Playlist(list) => {
                    if !playlist.is_empty() {
                        sink.clear();
                    }
                    for li in list {
                        playlist.push_back(li);
                    }
                }
                PlaybackEvent::PauseToggle => {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                _ => {}
            }
            if sink.empty() {
                if let Some(music_file) = playlist.pop_front() {
                    let file =
                        File::open(&music_file).expect("ERROR: can't open a file in {music_file}");

                    println!("DECODING: {}", &music_file);

                    match rodio::Decoder::new(file) {
                        Ok(source) => {
                            println!("NOW PLAYING : {}", &music_file);
                            sink.append(source)
                        }
                        Err(err) => {
                            eprintln!("ERROR: can't add {} into playlist {err}", &music_file)
                        }
                    }
                }
            }
        }
    });
}
