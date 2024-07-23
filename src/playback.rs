use std::{collections::VecDeque, fs::File, path::Path};

use rodio::{OutputStream, Sink};
use std::sync::mpsc::Receiver;

use crate::NowPlaying;

pub enum PlaybackEvent {
    Playlist(VecDeque<String>),
    PauseToggle,
    Add(String),
    Forward,
    Backward,
    Quit,
}

pub fn start_playing(rx: Receiver<PlaybackEvent>, now_playing: NowPlaying) {
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
                        playlist = list;
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
                        playlist = VecDeque::with_capacity(1);
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
                    if let Ok(mut song_name) = now_playing.try_write() {
                        let new_playing = extract_id(&music_file);
                        *song_name = new_playing;
                    }

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
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });
}

fn extract_id(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);

    let os_name = path.file_stem().map(|s| s.to_str())??;
    Some(os_name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steming_1() {
        let file_path = "music/12343.mp3";
        let name = extract_id(file_path).unwrap_or_default();
        dbg!(&name);

        assert_eq!("12343".to_owned(), name);
    }

    #[test]
    fn test_steming_2() {
        let file_path = "music/Linking Park.mp3";
        let name = extract_id(file_path).unwrap_or_default();

        assert_eq!("Linking Park".to_owned(), name);
    }

    #[test]
    fn test_steming_3() {
        let file_path = "music/Lar-uku.mp3";
        let name = extract_id(file_path).unwrap_or_default();

        assert_eq!("Lar-uku".to_owned(), name);
    }
}
