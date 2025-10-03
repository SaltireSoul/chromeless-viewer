#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let url = args.get(1).unwrap_or(&"https://copilot.microsoft.com/".to_string());

  tauri::Builder::default()
    .setup(move |app| {
      let window = app.get_window("main").unwrap();
      window.eval(&format!("window.location.replace('{}')", url)).unwrap();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}