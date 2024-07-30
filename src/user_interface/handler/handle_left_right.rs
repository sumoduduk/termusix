use crate::app::{screen::Screen, App};

pub fn hande_left(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            app.file_explorer.go_back();
        }
        _ => {}
    }
}

pub fn hande_right(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            app.file_explorer.enter_dir();
        }
        _ => {}
    }
}
