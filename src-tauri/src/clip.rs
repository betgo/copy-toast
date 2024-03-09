extern crate clipboard;

use std::thread;
use std::time::Duration;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use tauri::Window;

use crate::utils::hide_window;
use crate::utils::show_window;

pub fn copy_to_clipboard(window: &Window) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_contents = "".to_string();

    loop {
        let contents = match ctx.get_contents() {
            Ok(contents) => contents,
            Err(_) => last_contents.clone(),
        };
        if contents != last_contents {
            last_contents = contents;
            if window.is_visible().unwrap_or(false) {
                hide_window(&window);
            } else {
                show_window(&window);
                thread::sleep(Duration::from_secs(1));
                hide_window(&window)
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
