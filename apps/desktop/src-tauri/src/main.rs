//! Desktop entry point. Only wires Tauri to the engine command — no logic here.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::engine])
        .run(tauri::generate_context!())
}
