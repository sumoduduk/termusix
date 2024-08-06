use crate::app::{screen::Screen, App};

pub fn hande_left(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            let _ = app.file_explorer.go_back();
        }
        Screen::Playback => {
            app.button_prev();
        }
        _ => {
            app.seek_backward();
        }
    }
}

pub fn hande_right(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            let _ = app.file_explorer.enter_dir();
        }

        Screen::Playback => {
            app.button_next();
        }
        _ => {
            app.seek_forward();
        }
    }
}
