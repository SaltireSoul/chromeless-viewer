#![cfg_attr(windows, windows_subsystem = "windows")]

use tauri::{Builder, window::WindowBuilder};

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());
  let title = args.get(2).unwrap_or(&"Chromeless Viewer".to_string());
  let width = args.get(3).and_then(|w| w.parse::<f64>().ok()).unwrap_or(1600.0);
  let height = args.get(4).and_then(|h| h.parse::<f64>().ok()).unwrap_or(900.0);
  let x = args.get(5).and_then(|x| x.parse::<f64>().ok());
  let y = args.get(6).and_then(|y| y.parse::<f64>().ok());

  let redirect_url = format!("index.html?url={}", url);

  Builder::default()
    .setup(move |app| {
      let mut window = WindowBuilder::new(app, "main")
        .title(title)
        .inner_size(width, height)
        .resizable(true)
        .decorations(true)
        .url(redirect_url);

      if let (Some(x), Some(y)) = (x, y) {
        window = window.position(x, y);
      }

      window.build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}