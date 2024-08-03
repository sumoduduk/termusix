use std::path::PathBuf;

pub fn get_music_path() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    let music_dir = home_dir.join("Music");

    match music_dir.try_exists() {
        Ok(is_exist) => {
            if is_exist {
                Some(music_dir)
            } else {
                dirs::home_dir()
            }
        }
        Err(_) => dirs::home_dir(),
    }
}
