#![cfg_attr(windows, windows_subsystem = "windows")]

use std::{env, fs};
use serde::Deserialize;
// Added Size and Position types for Tauri window configuration
use tauri::{Builder, WebviewUrl, webview::WebviewWindowBuilder, Size, LogicalSize, Position, LogicalPosition};

#[derive(Deserialize)]
struct Config {
    url: String,
    title: String,
    width: f64,  // Changed to f64 to match standard LogicalSize primitives
    height: f64, // Changed to f64
    x: f64,      // Changed to f64 to match standard LogicalPosition primitives
    y: f64,      // Changed to f64
}

fn main() {
    // 1. Collect all arguments passed to the app (skipping the executable path)
    let args: Vec<String> = env::args().skip(1).collect();

    // 2. Try to load config from a JSON file path if specified in the arguments
    let config: Config = args
        .first()
        // Check if the first argument looks like a JSON configuration file
        .filter(|arg| arg.ends_with(".json")) 
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        // 3. Fallback: If no JSON file was provided (or loading it failed),
        // treat arguments as positional data (URL, Title, Width, Height, X, Y)
        .unwrap_or_else(|| {
            if args.len() >= 2 {
                Config {
                    url: args[0].clone(),
                    title: args[1].clone(),
                    // Fixed indices to step sequentially: 2, 3, 4, 5
                    width: args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1600.0),
                    height: args.get(3).and_then(|s| s.parse().ok()).unwrap_or(900.0),
                    x: args.get(4).and_then(|s| s.parse().ok()).unwrap_or(480.0),
                    y: args.get(5).and_then(|s| s.parse().ok()).unwrap_or(253.0),
                }
            } else {
                // 4. Ultimate Fallback: Default values
                Config {
                    url: "https://copilot.microsoft.com/".to_string(),
                    title: "Copilot".to_string(),
                    width: 1600.0,
                    height: 900.0,
                    x: 480.0,
                    y: 253.0,
                }
            }
        });

    Builder::default()
        .setup(move |app| {
            // Parse URL safely to avoid potential runtime panics
            let target_url = config.url.parse().unwrap_or_else(|_| {
                "https://copilot.microsoft.com/".parse().unwrap()
            });

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(target_url))
                .title(&config.title)
                // Corrected: Wrapped sizes and positions into standard Tauri structures
                .inner_size(Size::Logical(LogicalSize::new(config.width, config.height)))
                .position(Position::Logical(LogicalPosition::new(config.x, config.y)))
                .resizable(true)
                .decorations(true)
                .build()?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, _event| {});
}