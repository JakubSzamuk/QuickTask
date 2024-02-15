// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod macro_handler;

use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use macro_handler::MacroHandler;



struct AppState(Mutex<MacroHandler>);

#[derive(serde::Serialize)]
struct CurrentHandlerState {
    is_recording: bool,
    is_playing: bool,
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_recording(app_handler: tauri::State<AppState>) {
    println!("Starting to record");
    let _ = app_handler.0.lock().unwrap().start_recording();
    println!("status: {} {}", app_handler.0.lock().unwrap().is_recording, app_handler.0.lock().unwrap().is_playing)
}
#[tauri::command]
fn stop_recording(app_handler: tauri::State<AppState>) {
    println!("Stopping recording");
    let _ = app_handler.0.lock().unwrap().stop_recording();
}

#[tauri::command]
fn play_macro(app_handler: tauri::State<AppState>, speed: u8) {
    println!("{} is the speed", speed);
    let _ = app_handler.0.lock().unwrap().play_macro(&speed);
}
#[tauri::command]
fn get_handler_status(app_handler: tauri::State<AppState>) -> CurrentHandlerState {
    let app_handle = app_handler.0.lock().unwrap();
    CurrentHandlerState {
        is_recording: app_handle.is_recording,
        is_playing: app_handle.is_playing
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(AppState(Mutex::new(MacroHandler::new())))
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording, play_macro, get_handler_status])
        .run(tauri::generate_context!())
        .expect("error while running QuickRun");
}
