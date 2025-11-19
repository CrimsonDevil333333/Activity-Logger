# Activity-Logger

A powerful, cross-platform desktop application that tracks user activity including keystrokes, active windows, and screenshots. Features a modern web-based dashboard for viewing and managing all logged data.

## ✨ Features

### Core Functionality
- **📝 Keylogging**: Captures and logs keyboard inputs with timestamps and window context
- **🪟 Window Tracking**: Monitors and logs active window changes
- **📸 Screenshot Capture**: Automatic screenshot capture at configurable intervals
- **🎯 System Tray Integration**: Runs silently in the background with quick access via system tray

### Web Dashboard
- **📊 Statistics Dashboard**: Real-time overview of logs, screenshots, and disk usage
- **🔍 Search & Filter**: Instantly search and filter through all logs
- **🗑️ Clear Options**: Delete logs and screenshots with confirmation dialogs
- **⚙️ Config Editor**: Edit settings directly from the web interface
- **🔄 Auto-refresh**: Configurable refresh intervals (5s, 10s, 30s, or off)
- **🖼️ Screenshot Gallery**: Grid view with modal preview
- **🌐 Modern UI**: Dark theme with smooth animations and responsive design

### Configuration
- **⏱️ Inactivity Timeout**: Configurable timeout for logging pauses
- **🔧 Customizable Settings**: Log locations, screenshot intervals, server port, and more
- **🔔 Notifications**: Optional notifications for start, stop, and errors
- **⌨️ Hotkeys**: Configurable keyboard shortcuts (pause/resume, screenshot)

### Cross-Platform
- ✅ **Windows** (10 and later)
- ✅ **Linux** (X11 with system dependencies)
- ✅ **macOS** (experimental support)

## 🚀 Quick Start

### For End Users (Pre-built Package)

1. **Download** the latest release for your platform:
   - Windows: `Activity-Logger-Windows.zip`
   - Linux: `Activity-Logger-Linux.tar.gz`

2. **Extract** the archive to any location

3. **Run** the application:
   - Windows: Double-click `run.bat`
   - Linux: Run `./run.sh` in terminal

4. **Access Dashboard**: Open `http://localhost:8080` in your browser

5. **System Tray**: Look for the Activity Logger icon in your system tray for quick access

### For Developers

#### Prerequisites
- Rust 1.70 or later
- Cargo package manager

#### Platform-Specific Dependencies

**Linux:**
```bash
sudo apt-get install libx11-dev libappindicator3-dev xdotool libdbus-1-dev
```

**macOS:**
```bash
# No additional dependencies required
```

#### Build from Source

```bash
# Clone the repository
git clone https://github.com/CrimsonDevil333333/Activity-Logger.git
cd Activity-Logger

# Build release version
cargo build --release

# Run the application
cargo run --release
```

#### Create Distribution Package

**Windows:**
```powershell
.\build-package.ps1
```

**Linux:**
```bash
chmod +x build-package.sh
./build-package.sh
```

This creates a portable package in the `dist/` folder ready for distribution.

## ⚙️ Configuration

Edit `config.json` to customize the application:

```json
{
  "key_log_file": "keys.log",
  "window_log_file": "windows.log",
  "log_dir": "temp",
  "inactivity_timeout_secs": 300,
  "screenshot_enabled": true,
  "screenshot_interval_secs": 30,
  "screenshot_resolution": [1920, 1080],
  "server_port": 8080,
  "hotkeys": {
    "pause_resume": "Ctrl+Shift+P",
    "screenshot": "Ctrl+Shift+S"
  },
  "notification": {
    "on_start": true,
    "on_stop": true,
    "on_error": true
  },
  "summary_report": {
    "enabled": false,
    "interval_days": 1,
    "output_file": "summary.txt"
  }
}
```

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `key_log_file` | Filename for key logs | `keys.log` |
| `window_log_file` | Filename for window logs | `windows.log` |
| `log_dir` | Directory for logs ("temp" uses system temp) | `temp` |
| `inactivity_timeout_secs` | Seconds before pausing logging | `300` |
| `screenshot_enabled` | Enable screenshot capture | `true` |
| `screenshot_interval_secs` | Seconds between screenshots | `30` |
| `screenshot_resolution` | Max screenshot resolution [width, height] | `[1920, 1080]` |
| `server_port` | Web dashboard port | `8080` |

## 🌐 Web Dashboard

Access the dashboard at `http://localhost:8080` (or your configured port).

### Dashboard Sections

1. **📊 Dashboard**: Overview with statistics cards showing:
   - Total key logs
   - Total window logs
   - Total screenshots
   - Disk usage

2. **📝 Key Logs**: Searchable table of all keystroke logs with:
   - Timestamp
   - Active window
   - Input text
   - Real-time search/filter
   - Clear logs option

3. **🪟 Window Logs**: Searchable table of window changes with:
   - Timestamp
   - Window title
   - Real-time search/filter
   - Clear logs option

4. **📸 Screenshots**: Gallery view with:
   - Thumbnail grid
   - Click to view full size
   - Clear all option

5. **⚙️ Settings**: Edit configuration directly:
   - Log file paths
   - Timeouts and intervals
   - Screenshot settings
   - Server port
   - Save and restart to apply

### Dashboard Features

- **🔍 Search**: Real-time filtering across all logs
- **🔄 Auto-refresh**: Configurable refresh intervals
- **🗑️ Clear Data**: Delete logs/screenshots with confirmation
- **📱 Responsive**: Works on desktop and mobile browsers
- **🎨 Modern UI**: Dark theme with smooth animations

## 📁 Log Output

### Key Logs (JSON Lines format)
```json
{"timestamp":"2024-01-20 10:30:45","window":"Visual Studio Code","input":"Hello World"}
{"timestamp":"2024-01-20 10:30:50","window":"Chrome","input":"activity logger"}
```

### Window Logs (JSON Lines format)
```json
{"timestamp":"2024-01-20 10:30:45","title":"Visual Studio Code - main.rs"}
{"timestamp":"2024-01-20 10:31:00","title":"Google Chrome - Activity Logger"}
```

### Screenshots
- Saved as PNG files in `screenshots/` subdirectory
- Filename format: `screenshot_YYYY-MM-DD_HH-MM-SS.png`

## 🎯 System Tray Menu

Right-click the system tray icon for quick access:

- **Open Dashboard**: Opens web dashboard in browser
- **Show Logs**: Opens log directory in file explorer
- **Open Config**: Opens config.json in default editor
- **Open Screenshots**: Opens screenshots folder
- **Clear Logs**: Clears all log files
- **Clear Screenshots**: Deletes all screenshots
- **Clear Everything**: Clears both logs and screenshots
- **Quit**: Exits the application

## 📦 Distribution

### Creating Releases

1. **Tag your version**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **GitHub Actions** will automatically:
   - Build for Windows and Linux
   - Create distribution packages
   - Create a GitHub release
   - Upload packages to the release

3. **Users download** from the Releases page

### Package Contents

Each distribution includes:
- Executable (activity_logger.exe or activity_logger)
- Configuration file (config.json)
- Launcher script (run.bat or run.sh)
- Documentation (README.md)
- Application icon

## 🔒 Privacy & Security

- **Local Only**: All data is stored locally on your machine
- **No Network**: No data is sent to external servers
- **User Control**: Full control over what is logged and when
- **Clear Options**: Easy deletion of all logged data

## 🛠️ Development

### Project Structure
```
Activity-Logger/
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── tracker.rs        # Activity tracking logic
│   ├── tray.rs           # System tray integration
│   ├── server.rs         # Web server & API
│   └── platform/         # Platform-specific code
├── assets/
│   ├── index.html        # Dashboard frontend
│   ├── icon.ico          # Windows icon
│   └── icon.png          # Linux icon
├── .github/
│   └── workflows/
│       └── rust.yml      # CI/CD pipeline
├── build-package.ps1     # Windows packaging script
├── build-package.sh      # Linux packaging script
└── config.json           # Default configuration
```

### API Endpoints

- `GET /` - Dashboard UI
- `GET /api/logs/keys` - Fetch key logs
- `GET /api/logs/windows` - Fetch window logs
- `GET /api/screenshots` - List screenshots
- `GET /api/stats` - Get statistics
- `GET /api/config` - Get configuration
- `POST /api/config` - Update configuration
- `DELETE /api/logs/keys` - Clear key logs
- `DELETE /api/logs/windows` - Clear window logs
- `DELETE /api/screenshots` - Clear screenshots

## 📝 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -m "Add feature-name"`
4. Push to branch: `git push origin feature-name`
5. Submit a pull request

## 🐛 Issues & Support

For bugs, feature requests, or questions:
- Open an issue on GitHub
- Visit: https://github.com/CrimsonDevil333333/Activity-Logger

## 🙏 Acknowledgments

- Built with Rust for performance and reliability
- Uses Actix-web for the web server
- System tray integration via tray-item
- Cross-platform support with platform-specific crates

---

**⚠️ Disclaimer**: This tool is for personal use and productivity tracking. Ensure you comply with local laws and regulations regarding activity logging and monitoring.