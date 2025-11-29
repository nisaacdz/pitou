// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
use cmds::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            general_folders,
            default_folder,
            drives,
            children,
            thrash_items,
            clipboard_empty,
            copy,
            cut,
            paste,
            delete,
            open,
            open_with,
            archive,
            create_dir,
            create_file,
            rename,
            extract,
            search,
            terminate_search,
            search_msg,
            is_searching,
            transfer_sessions,
            transfer_session_with_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
