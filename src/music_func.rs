mod download;

use download::download_music;
use tokio::sync::mpsc::Sender;

use crate::file_ops::check_file_exist;

pub async fn play(music_paths: Vec<String>, tx: Sender<String>) -> eyre::Result<()> {
    let mut handles = Vec::with_capacity(2);

    let handle_async = tokio::spawn(async move {
        for music_path_str in music_paths.iter() {
            let music_path = ["music/", music_path_str, ".mp3"].concat();

            if check_file_exist(&music_path).await.is_some() {
                if let Err(err) = tx.send(music_path).await {
                    eprintln!("ERROR: {err}");
                };
            } else {
                let res_download = download_music(music_path_str, &music_path).await;

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

    // let handle_sync = tokio::task::spawn_blocking(move || {
    // });
    //
    // handles.push(handle_sync);

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
