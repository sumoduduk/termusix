use std::os::unix::fs::MetadataExt;

use tokio::fs::metadata;

pub async fn check_file_exist(file_name: &str) -> Option<&str> {
    let meta_res = metadata(file_name).await;
    match meta_res {
        Ok(meta_res) => {
            let size = meta_res.size();
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
