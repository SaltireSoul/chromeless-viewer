#![cfg_attr(windows, windows_subsystem = "windows")]

use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    url: String,
    title: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

fn main() {
    let config: Config = match fs::read_to_string("config.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok()) {
            Some(cfg) => cfg,
            None => {
                eprintln!("Failed to load config.json");
                return;
            }
    };


    tauri::Builder::default()
        .setup(|app| {
            tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::External(config.url.parse().unwrap())
            )
            .title(&config.title)
            .inner_size(config.width, config.height)
            .position(config.x, config.y)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}