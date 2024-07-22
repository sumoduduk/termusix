use std::fs::metadata as metada_sync;
use tokio::fs::metadata;

pub async fn check_file_exist(file_name: &str) -> Option<&str> {
    let meta_res = metadata(file_name).await;
    match meta_res {
        Ok(meta_res) => {
            let size = meta_res.len();
            let is_file = meta_res.is_file();

            if size > 0 && is_file {
                Some(file_name)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn check_file_exist_sync(file_name: &str) -> Option<&str> {
    let meta_res = metada_sync(file_name);
    match meta_res {
        Ok(meta_res) => {
            let size = meta_res.len();
            let is_file = meta_res.is_file();

            if size > 0 && is_file {
                Some(file_name)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

// pub async fn count_file(folder_path: &str) -> eyre::Result<usize> {
//     let mut count = 0;
//
//     let mut dir = fs::read_dir(folder_path).await?;
//     while let Some(entry) = dir.next_entry().await? {
//         let file_type = entry.file_type().await?;
//         if file_type.is_file() {
//             count += 1;
//         }
//     }
//
//     Ok(count)
// }
