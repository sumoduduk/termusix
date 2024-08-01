use std::{collections::VecDeque, path::PathBuf, sync::Arc};

use crate::{app::App, file_ops::send_id_file_exist, playback::PlaybackEvent};

pub async fn play_and_download(app: &App) {
    let indx = app.tabs_playlist.selected();

    let Some(title) = app.playlist.get_playlist_tittle(indx) else {
        return;
    };

    let Some(pl) = app.list_playlist_song_id(indx) else {
        return;
    };

    let sender = app.tx_playback.clone();

    if title.contains("local") {
        let song_paths: VecDeque<PathBuf> = pl
            .iter()
            .map(PathBuf::from)
            .filter(|f| f.is_file())
            .collect();

        let _ = sender.send(PlaybackEvent::Playlist(song_paths));
    } else {
        let home = dirs::home_dir().expect("need home dir");
        let music_dir = dirs::audio_dir().unwrap_or(home.join("Music"));

        let arc_dir = Arc::new(music_dir);

        for id_song in pl {
            let dir_clone = Arc::clone(&arc_dir);
            let sender2 = sender.clone();
            tokio::spawn(async move {
                let _ = send_id_file_exist(&id_song, &dir_clone, sender2).await;
            });
        }
    }

    // if let Some((list, len)) = app.list_id_downloaded_first(indx) {
    //     tokio::spawn(async move {
    //         let downloaded: VecDeque<String> = list[..len]
    //             .to_vec()
    //             .iter()
    //             .map(|id| concat_file(id))
    //             .collect();
    //
    //         if !downloaded.is_empty() {
    //             let _ = send.send(PlaybackEvent::Playlist(downloaded));
    //         }
    //         let not_downloaded = &list[len..].to_vec();
    //         for id in not_downloaded {
    //             let res = download_music(id).await;
    //             if res.is_ok() {
    //                 let _ = send.send(PlaybackEvent::Add(concat_file(id)));
    //                 time::sleep(Duration::from_secs(30)).await;
    //             }
    //         }
    //     });
    // }
}

fn concat_file(id: &str) -> String {
    ["music/", id, ".mp3"].concat()
}
