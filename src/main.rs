#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Result;
use tauri::App;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window =
        tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
            .build()?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
