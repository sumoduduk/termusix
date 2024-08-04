use super::{InfoMusicPlaylist, MusicInfo, MusicPlaylist, Playlist};
use crate::{PlaylistInfo, VideoResult};
use eyre::OptionExt;
use indexmap::IndexMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const URL: &str = "https://inv.tux.pizza/api/v1/playlists";

pub async fn get_playlist(playlist: &mut Playlist, url: &str) -> eyre::Result<()> {
    let id_playlist = extract_playlist_id(url)?;

    let instance = format!("{}/{}", URL, id_playlist);

    let data_json: PlaylistInfo = reqwest::get(instance).await?.json().await?;

    let info_playlist = convert_playlist(&data_json);

    playlist.0.insert(id_playlist.to_owned(), info_playlist);

    Ok(())
}

pub async fn save_id(playlist: &mut Playlist, id: &str) -> eyre::Result<()> {
    let instance = format!("{}/{}", URL, id);
    let data_json: PlaylistInfo = reqwest::get(instance).await?.json().await?;
    let info_playlist = convert_playlist(&data_json);

    playlist.0.insert(id.to_owned(), info_playlist);

    Ok(())
}

fn convert_playlist(data_json: &PlaylistInfo) -> InfoMusicPlaylist {
    let playlist_title = &data_json.title;
    let videos = &data_json.videos;

    let music_list = map_music_info(videos);

    InfoMusicPlaylist {
        playlist_title: playlist_title.to_owned(),
        music_list,
    }
}

fn map_music_info(videos: &[VideoResult]) -> MusicInfo {
    let mut music_list: MusicInfo = IndexMap::with_capacity(videos.len());

    videos.iter().for_each(|info_vid| {
        let music_id = info_vid.video_id.to_owned();
        let music_title = info_vid.title.to_owned();

        music_list.insert(music_id, music_title);
    });
    music_list
}

pub fn save_file_json(playlist: &MusicPlaylist, path: &Path) -> eyre::Result<()> {
    let json_str = serde_json::to_string_pretty(&playlist)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

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
    fn test_save_split_url_test() -> eyre::Result<()> {
        let url = "https://www.youtube.com/playlist?list=PL8noWinfnxi2uEgElXejICKtZY0_sAYK5";
        let id = extract_playlist_id(url)?;

        assert_eq!(id, "PL8noWinfnxi2uEgElXejICKtZY0_sAYK5");
        Ok(())
    }

    #[test]
    fn test_save_convert() -> eyre::Result<()> {
        let download_info = r#"
        {
          "title": "indo",
          "playlistId": "PL8noWinfnxi2U48fYh2tkHLesDBUEQsJN",
          "author": "comodo musix",
          "videoCount": 87,
          "videos": [{
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
        }
        "#;

        let data_download: PlaylistInfo = serde_json::from_str(download_info)?;

        let title = &data_download.title;
        let info_music_playlist = convert_playlist(&data_download);

        let target_music = IndexMap::from([
            ("dGcGbF4ex5o".to_string(), "Dan".to_string()),
            ("oOXba6xE41Q".to_string(), "Kita".to_string()),
            ("BNYJ7NcQ7no".to_string(), "Raja".to_string()),
        ]);

        let target = InfoMusicPlaylist {
            playlist_title: title.to_owned(),
            music_list: target_music,
        };

        assert_eq!(info_music_playlist, target);

        Ok(())
    }

    #[test]
    fn test_save_map_music_info() -> eyre::Result<()> {
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

        let target = IndexMap::from([
            ("dGcGbF4ex5o".to_string(), "Dan".to_string()),
            ("oOXba6xE41Q".to_string(), "Kita".to_string()),
            ("BNYJ7NcQ7no".to_string(), "Raja".to_string()),
        ]);

        assert_eq!(music_info, target);

        Ok(())
    }

    #[test]
    fn test_save_main() -> eyre::Result<()> {
        let raw = std::fs::read_to_string("playlist.json")?;

        let data_download: PlaylistInfo = serde_json::from_str(&raw)?;

        let playlist_id = &data_download.playlist_id;

        let info_music = convert_playlist(&data_download);

        let mut music_playlist = IndexMap::with_capacity(1);

        music_playlist.insert(playlist_id.to_owned(), info_music);

        save_file_json(&music_playlist)?;

        let file_string = std::fs::read_to_string("music.json")?;

        let data: MusicPlaylist = serde_json::from_str(&file_string)?;

        let target = data.get(playlist_id);

        assert!(target.is_some());

        Ok(())
    }
}
