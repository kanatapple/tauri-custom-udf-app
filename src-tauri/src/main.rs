// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let data_dir_path = app.path().data_dir();
            let data_dir_path = data_dir_path.unwrap_or_else(|_| match std::env::var("APPDATA") {
                Ok(path) => PathBuf::from(path),
                Err(_) => PathBuf::new(),
            });
            let data_directory = data_dir_path.join("tauri-custom-udf-app");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .data_directory(data_directory)
                .build()
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
