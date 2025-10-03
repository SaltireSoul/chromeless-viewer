#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::window::WindowBuilder;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let url = args.get(1).cloned().unwrap_or_else(|| "https://copilot.microsoft.com/".to_string());
  let title = args.get(2).cloned().unwrap_or_else(|| "Chromeless Viewer".to_string());
  let redirect_url = format!("index.html?url={}", url);

  tauri::Builder::default()
    .setup(move |app| {
      WindowBuilder::new(app, "main")
        .title(&title)
        .inner_size(1600.0, 900.0)
        .resizable(true)
        .decorations(true)
        .url(redirect_url)
        .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}