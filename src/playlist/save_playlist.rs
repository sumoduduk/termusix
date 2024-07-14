use super::{MusicInfo, MusicPlaylist};
use crate::{PlaylistInfo, VideoResult};
use eyre::OptionExt;
use std::fs::OpenOptions;
use std::io::Write;

const URL: &str = "https://inv.tux.pizza/api/v1/playlists";

pub async fn get_playlist(url: &str) -> eyre::Result<MusicPlaylist> {
    let id_playlist = extract_playlist_id(url)?;

    let instance = format!("{}/{}", URL, id_playlist);

    let data_json: PlaylistInfo = reqwest::get(instance).await?.json().await?;

    let playlist_title = &data_json.title;

    let videos = &data_json.videos;

    let music_list = map_music_info(videos);

    let music_playlist = MusicPlaylist {
        playlist_id: id_playlist.to_owned(),
        playlis_title: playlist_title.to_owned(),
        music_list,
    };

    Ok(music_playlist)
}

fn map_music_info(videos: &[VideoResult]) -> Vec<MusicInfo> {
    let music_list: Vec<MusicInfo> = videos
        .iter()
        .map(|info_vid| MusicInfo {
            music_id: info_vid.video_id.to_owned(),
            music_title: info_vid.title.to_owned(),
        })
        .collect();
    music_list
}

pub fn save_file_json(playlist: &[MusicPlaylist]) -> eyre::Result<()> {
    let json_str = serde_json::to_string(playlist)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("music.json")?;

    file.write_all(json_str.as_bytes())?;

    Ok(())
}

fn extract_playlist_id(url: &str) -> eyre::Result<&str> {
    let right_side = url
        .split('=')
        .nth(1)
        .ok_or_eyre("ERROR: can't get right side of URL")?;

    Ok(right_side)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_url_test() -> eyre::Result<()> {
        let url = "https://www.youtube.com/playlist?list=PL8noWinfnxi2uEgElXejICKtZY0_sAYK5";
        let id = extract_playlist_id(url)?;

        assert_eq!(id, "PL8noWinfnxi2uEgElXejICKtZY0_sAYK5");
        Ok(())
    }

    #[test]
    fn test_map_music_info() -> eyre::Result<()> {
        let vid_info = r#"
        [{
          "title": "Dan",
          "videoId": "dGcGbF4ex5o",
          "author": "Sony Music Entertainment Indonesia",
          "lengthSeconds": 284
        },
        {
          "title": "Kita",
          "videoId": "oOXba6xE41Q",
          "author": "Sony Music Entertainment Indonesia",
          "lengthSeconds": 225
        },
        {
          "title": "Raja",
          "videoId": "BNYJ7NcQ7no",
          "author": "Sony Music Entertainment Indonesia",
          "lengthSeconds": 287
        }]
        "#;

        let data: Vec<VideoResult> = serde_json::from_str(vid_info)?;

        let music_info = map_music_info(&data);

        let target = vec![
            MusicInfo {
                music_id: "dGcGbF4ex5o".to_string(),
                music_title: "Dan".to_string(),
            },
            MusicInfo {
                music_id: "oOXba6xE41Q".to_string(),
                music_title: "Kita".to_string(),
            },
            MusicInfo {
                music_id: "BNYJ7NcQ7no".to_string(),
                music_title: "Raja".to_string(),
            },
        ];

        assert_eq!(music_info, target);

        Ok(())
    }
}
