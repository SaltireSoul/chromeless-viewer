#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, Url};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());
  let title = args.get(2).unwrap_or(&"Chromeless Viewer".to_string());
  let redirect_url = format!("index.html?url={}", url);

  Builder::default()
    .with_url(Url::App(redirect_url.into()))
    .setup(move |app| {
      let window = app.get_window("main").unwrap();
      window.set_title(title).ok();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}