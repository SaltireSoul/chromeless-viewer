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
    let config: Config = serde_json::from_str(&fs::read_to_string("config.json").unwrap()).unwrap();

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