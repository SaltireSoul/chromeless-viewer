#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, Manager, window::WindowBuilder};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let url = args.get(1).cloned().unwrap_or_else(|| "https://copilot.microsoft.com/".to_string());
  let title = args.get(2).cloned().unwrap_or_else(|| "Chromeless Viewer".to_string());

  let redirect_url = format!("index.html?url={}", url);

  Builder::default()
    .setup(move |app| {
      let mut window = WindowBuilder::new(app, "main")
        .title(&title)
        .url(redirect_url)
        .inner_size(1600.0, 900.0)
        .resizable(true)
        .decorations(true);

      window.build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}