#![cfg_attr(windows, windows_subsystem = "windows")]

use std::{env, fs};
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
                let args: Vec<String> = env::args().skip(1).collect();
                if args.len() >= 2 {
                    Config {
                        url: args[0].clone(),
                        title: args[1].clone(),
                        width: args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1600),
                        height: args.get(3).and_then(|s| s.parse().ok()).unwrap_or(900),
                        x: args.get(4).and_then(|s| s.parse().ok()).unwrap_or(480),
                        y: args.get(5).and_then(|s| s.parse().ok()).unwrap_or(253),
                    }
                } else {
                    Config {
                        url: "https://copilot.microsoft.com/".to_string(),
                        title: "Copilot".to_string(),
                        width: 1600,
                        height: 900,
                        x: 480,
                        y: 253,
                    }
                }
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
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, _event| {});
}