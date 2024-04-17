// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
use cmds::*;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            general_folders,
            default_folder,
            drives,
            children,
            thrash_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
