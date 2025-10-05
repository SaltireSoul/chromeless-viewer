# Chromeless Viewer

Chromeless Viewer is a lightweight desktop application that opens a single customizable webview window. It’s ideal for dashboards, kiosk displays, or focused browsing experiences.

## 🤖 AI Attribution

> ⚠️ This project was written with the assistance of Microsoft Copilot.  
All core logic, configuration, and documentation were generated through AI guidance.  
The application reflects Copilot's output across its design, implementation, and structure.

## 🚀 Features

- Loads any external URL in a native window
- Configurable via `config.json` or command-line arguments
- Supports window decorations and resizing
- Defaults to Copilot if no config is provided

## 🛠 Usage

### 1. Launch with `config.json`

Place a `config.json` file next to `chromeless-viewer.exe` with the following structure:

```json
{
  "url": "https://example.com",
  "title": "My App",
  "width": 1200,
  "height": 800,
  "x": 100,
  "y": 100
}
```

Then run:
```bash
chromeless-viewer.exe
```

### 2. Launch with Command-Line Arguments

You can also pass arguments directly:
```bash
chromeless-viewer.exe "https://example.com" "My App" 1200 800 100 100
```

**Argument order:**

1. `url` – the website to load (required)
2. `title` – window title (required)
3. `width` – window width (optional, default: `1600`)
4. `height` – window height (optional, default: `900`)
5. `x` – window x position (optional, default: `480`)
6. `y` – window y position (optional, default: `253`)

### 3. Default Behavior

If neither `config.json` nor arguments are provided, the app launches with:

- **URL**: `https://copilot.microsoft.com/`
- **Title**: `Copilot`
- **Size**: `1600x900`
- **Position**: `x=480, y=253`

## 📦 Output

The application runs as a single executable:  
**`chromeless-viewer.exe`**

No installation required. Just double-click or launch via terminal.

## 📝 License

This project is licensed under the [MIT License](LICENSE).
