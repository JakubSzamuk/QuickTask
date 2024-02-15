// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod macro_handler;
use macro_handler::MacroHandler;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_recording(app_handler: tauri::State<MacroHandler>) {
    
}
#[tauri::command]
fn stop_recording(app_handler: tauri::State<MacroHandler>) {
    
}

#[tauri::command]
fn play_macro(app_handler: tauri::State<MacroHandler>, speed: u8) {
    
}

fn main() {
    tauri::Builder::default()
        .manage(MacroHandler::new())
        .invoke_handler(tauri::generate_handler![start_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
