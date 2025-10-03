#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, WindowUrl};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());

  let redirect_url = format!("index.html?url={}", url);

  Builder::default()
    .run(tauri::generate_context!().with_url(WindowUrl::App(redirect_url.into())))
    .expect("error while running tauri application");
}