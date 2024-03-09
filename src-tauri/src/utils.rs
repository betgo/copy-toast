use tauri::Window;

pub fn hide_window(window: &Window) {
    window.hide().unwrap();
}

pub fn show_window(window: &Window) {
    window.show().unwrap();
}
