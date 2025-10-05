#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Manager, Builder};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());

  Builder::default()
    .setup(move |app| {
      let window = app.get_webview_window("main").unwrap();
      window.eval(&format!("window.location.replace('{}')", url)).unwrap();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}