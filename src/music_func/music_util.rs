pub fn get_path(id: &str) -> String {
    ["music/", id, ".mp3"].concat()
}
