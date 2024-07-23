use std::collections::VecDeque;

use crate::{app::App, download::download_music, playback::PlaybackEvent};
use tokio::time::{self, Duration};

pub async fn play_and_download(app: &App) {
    let indx = app.tabs_playlist.selected();
    let send = app.tx_playback.clone();
    if let Some((list, len)) = app.list_id_downloaded_first(indx) {
        tokio::spawn(async move {
            let downloaded: VecDeque<String> = list[..len]
                .to_vec()
                .iter()
                .map(|id| concat_file(id))
                .collect();

            if !downloaded.is_empty() {
                let _ = send.send(PlaybackEvent::Playlist(downloaded));
            }
            let not_downloaded = &list[len..].to_vec();
            for id in not_downloaded {
                let res = download_music(id).await;
                if res.is_ok() {
                    let _ = send.send(PlaybackEvent::Add(concat_file(id)));
                    time::sleep(Duration::from_secs(30)).await;
                }
            }
        });
    }
}

fn concat_file(id: &str) -> String {
    ["music/", id, ".mp3"].concat()
}
