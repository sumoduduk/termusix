use std::{collections::VecDeque, path::PathBuf};

use rodio::{OutputStream, Sink};
use std::sync::mpsc::Receiver;

use crate::{file_ops::decode_file, NowPlaying};

pub enum PlaybackEvent {
    Playlist((usize, VecDeque<PathBuf>)),
    PauseToggle,
    Add(PathBuf),
    TrackPlay(usize),
    Forward,
    Backward,
    DeleteTrack(usize),
    Quit,
}

pub fn start_playing(rx: Receiver<PlaybackEvent>, now_playing: NowPlaying) {
    tokio::task::spawn_blocking(move || {
        let (_stream, stram_handle) =
            OutputStream::try_default().expect("ERROR: error getting OutputStream");
        let sink = Sink::try_new(&stram_handle).expect("ERROR: play new sink");

        let mut playlist = VecDeque::new();
        let mut is_played = true;
        let mut playlist_id = 0;
        let mut song_id = 0;

        while is_played {
            if let Ok(evt) = rx.try_recv() {
                match evt {
                    PlaybackEvent::Playlist((pl_id, list)) => {
                        playlist = list;
                        playlist_id = pl_id;

                        if let Ok(mut song_name) = now_playing.try_write() {
                            song_name.playlist_id = Some(playlist_id);
                        }

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
                    PlaybackEvent::TrackPlay(s_id) => {
                        if song_id != s_id {
                            song_id = s_id;
                            sink.clear();
                        }
                    }
                    PlaybackEvent::Quit => {
                        playlist.clear();
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
                    }

                    PlaybackEvent::DeleteTrack(num) => {
                        if song_id == num {
                            sink.clear();
                        }

                        playlist.remove(num);
                    } // _ => {}
                }
            }

            if sink.empty() {
                if let Some(music_file) = playlist.get(song_id) {
                    if let Ok(mut song_name) = now_playing.try_write() {
                        song_name.song_title = music_file.to_str().map(|s| s.to_owned());
                    }

                    match decode_file(music_file) {
                        Ok(decoded) => {
                            sink.append(decoded);
                            sink.play();
                        }
                        Err(_) => {}
                    }

                    if song_id > playlist.len() - 1 {
                        song_id = 0;
                    } else {
                        song_id += 1;
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    fn extract_id(file_name: &Path) -> Option<String> {
        let os_name = file_name.file_stem().map(|s| s.to_str())??;
        Some(os_name.to_owned())
    }

    #[test]
    fn test_steming_1() {
        let file_path = Path::new("music/12343.mp3");
        let name = extract_id(file_path).unwrap_or_default();
        dbg!(&name);

        assert_eq!("12343".to_owned(), name);
    }

    #[test]
    fn test_steming_2() {
        let file_path = Path::new("music/Linking Park.mp3");
        let name = extract_id(file_path).unwrap_or_default();

        assert_eq!("Linking Park".to_owned(), name);
    }

    #[test]
    fn test_steming_3() {
        let file_path = Path::new("music/Lar-uku.mp3");
        let name = extract_id(file_path).unwrap_or_default();

        assert_eq!("Lar-uku".to_owned(), name);
    }
}
