#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, Manager};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  // Use owned Strings to avoid borrowing issues
  let url = args.get(1).cloned().unwrap_or_else(|| "https://copilot.microsoft.com/".to_string());
  let title = args.get(2).cloned().unwrap_or_else(|| "Chromeless Viewer".to_string());

  Builder::default()
    .setup(move |app| {
      if let Some(window) = app.get_window("main") {
        window.set_title(&title).ok();
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}