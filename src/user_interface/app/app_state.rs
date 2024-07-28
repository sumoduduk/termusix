use std::path::PathBuf;

use ratatui::widgets::ListState;
use tokio::fs;

#[derive(Default)]
pub struct AddSongState {
    folder_file: Vec<PathBuf>,
    add_paths: Vec<PathBuf>,
    pub file_scroll: ListState,
    pub music_scroll: ListState,
}

impl AddSongState {
    pub async fn load_folder(&mut self, file_path: &str) -> eyre::Result<()> {
        let mut path_files = fs::read_dir(file_path).await?;
        while let Some(path_file) = path_files.next_entry().await? {
            let path = path_file.path();

            let ext = path.extension().map(|ex| ex.to_str().unwrap_or(""));
            if let Some(ext) = ext {
                if ext == "mp3" || ext == "mp4" {
                    self.folder_file.push(path_file.path())
                }
            }
        }

        Ok(())
    }

    pub fn serve_folder_file(&self) -> Vec<&str> {
        let folder = &self.folder_file;
        let str_folder: Vec<&str> = folder
            .iter()
            .filter_map(|p| p.file_name().and_then(|os_str| os_str.to_str()))
            .collect();
        str_folder
    }

    pub fn serve_add_paths(&self) -> Vec<&str> {
        let folder = &self.add_paths;
        let str_folder: Vec<&str> = folder
            .iter()
            .filter_map(|p| p.file_name().and_then(|os_str| os_str.to_str()))
            .collect();
        str_folder
    }

    pub fn swap_to_folder_file(&mut self, index: usize) {
        let path = self.delete_add_paths(index);
        self.add_folder_file(path);
    }

    pub fn swap_to_add_paths(&mut self, index: usize) {
        let path = self.delete_folder_file(index);
        self.add_paths_value(path);
    }

    pub fn clear_folder(&mut self) {
        self.folder_file.clear();
    }

    pub fn clear_add_paths(&mut self) {
        self.add_paths.clear();
    }

    fn add_paths_value(&mut self, path: PathBuf) {
        self.add_paths.push(path)
    }

    fn add_folder_file(&mut self, path: PathBuf) {
        self.folder_file.push(path)
    }

    fn delete_folder_file(&mut self, index: usize) -> PathBuf {
        self.folder_file.swap_remove(index)
    }

    fn delete_add_paths(&mut self, index: usize) -> PathBuf {
        self.add_paths.swap_remove(index)
    }
}
