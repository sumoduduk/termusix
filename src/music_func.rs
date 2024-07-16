mod download;

use download::download_music;
use rodio::{OutputStream, Sink};
use std::{fs::File, path::Path};
use tokio::sync::mpsc;

use crate::file_ops::check_file_exist;

pub async fn play(music_paths: Vec<String>) -> eyre::Result<()> {
    let mut handles = Vec::with_capacity(2);

    let (tx, mut rx) = mpsc::channel(10);
    let handle_async = tokio::spawn(async move {
        for music_path_str in music_paths.iter() {
            let music_path = ["music/", music_path_str, ".mp3"].concat();

            if check_file_exist(&music_path).await.is_some() {
                if let Err(err) = tx.send(music_path).await {
                    eprintln!("ERROR: {err}");
                };
            } else {
                let res_download = download_music(music_path_str).await;

                match res_download {
                    Ok(_) => {
                        if let Err(err) = tx.send(music_path).await {
                            eprintln!("ERROR: {err}");
                        };
                    }
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
    });
    handles.push(handle_async);

    let handle_sync = tokio::task::spawn_blocking(move || {
        let (_stream, stram_handle) =
            OutputStream::try_default().expect("ERROR: error getting OutputStream");
        let sink = Sink::try_new(&stram_handle).expect("ERROR: play new sink");

        while let Some(music_file) = rx.blocking_recv() {
            println!("append {} to playlist", &music_file);
            let file = File::open(&music_file).expect("ERROR: can't open a file in {music_file}");

            match rodio::Decoder::new(file) {
                Ok(source) => sink.append(source),
                Err(err) => eprintln!("ERROR: can't add {} into playlist {err}", &music_file),
            }
        }
        sink.sleep_until_end();
    });

    handles.push(handle_sync);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
