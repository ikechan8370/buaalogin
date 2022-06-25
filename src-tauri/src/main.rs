#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod command;
pub mod encrypt;

use crate::command::{login, logout, fetch_option, update_option};
use tauri_plugin_autostart::MacosLauncher;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, logout, fetch_option, update_option])
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            false,
        ))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
