#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, WindowBuilder, WindowUrl};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());
  let redirect_url = format!("index.html?url={}", url);

  Builder::default()
    .setup(move |app| {
      WindowBuilder::new(
        app,
        "main",
        WindowUrl::App(redirect_url.into())
      )
      .title("Chromeless Viewer")
      .inner_size(1600.0, 900.0)
      .resizable(true)
      .decorations(true)
      .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}