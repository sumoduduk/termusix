use std::{collections::VecDeque, path::PathBuf, time::Duration};

use rodio::{OutputStream, Sink};
use std::sync::mpsc::Receiver;

use crate::{file_ops::decode_file, NowPlaying};
use std::path::Path;

pub enum PlaybackEvent {
    Playlist((usize, VecDeque<PathBuf>)),
    PauseToggle,
    Add(PathBuf),
    TrackPlay(usize),
    Forward,
    Backward,
    SeekForward,
    SeekBackward,
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
        let mut song_id = 0;

        while is_played {
            if let Ok(evt) = rx.try_recv() {
                match evt {
                    PlaybackEvent::Playlist((pl_id, list)) => {
                        playlist = list;

                        if let Ok(mut song_name) = now_playing.try_write() {
                            song_name.playlist_id = Some(pl_id);
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
                        song_id = s_id;
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
                            let curr = current_playing_index(song_id, playlist.len());

                            song_id = curr.saturating_sub(1);
                            sink.clear();
                        }
                    }

                    PlaybackEvent::DeleteTrack(num) => {
                        if current_playing_index(song_id, playlist.len()) == num {
                            sink.clear();
                        }

                        playlist.remove(num);
                    }
                    PlaybackEvent::SeekForward => {
                        if !sink.empty() {
                            let dur = sink.get_pos();
                            let _ = sink.try_seek(dur.saturating_add(Duration::from_secs(2)));
                        }
                    }
                    PlaybackEvent::SeekBackward => {
                        if !sink.empty() {
                            let dur = sink.get_pos();
                            let _ = sink.try_seek(dur.saturating_sub(Duration::from_secs(5)));
                        }
                    } // _ => {}
                }
            }

            if sink.empty() {
                if let Some(music_file) = playlist.get(song_id) {
                    if let Ok(mut song_name) = now_playing.try_write() {
                        song_name.song_title = extract_id(music_file).map(|s| s.to_owned());
                    }

                    match decode_file(music_file) {
                        Ok(decoded) => {
                            sink.append(decoded);
                            sink.play();
                        }
                        Err(_) => {}
                    }

                    if song_id >= playlist.len() - 1 {
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

fn current_playing_index(song_id: usize, len: usize) -> usize {
    match song_id {
        0 => len.saturating_sub(1),
        _ => song_id.saturating_sub(1),
    }
}

fn extract_id(file_name: &Path) -> Option<&str> {
    let os_name = file_name.file_stem().and_then(|s| s.to_str());
    os_name.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

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
