use std::path::PathBuf;

pub fn get_music_path() -> Option<PathBuf> {
    dirs::audio_dir().or(dirs::home_dir())
}
