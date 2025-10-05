#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, Manager};

fn main() {
  Builder::default()
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      window.eval("window.location.replace('https://copilot.microsoft.com/')").unwrap();
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run();
}