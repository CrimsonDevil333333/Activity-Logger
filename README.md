# Activity-Logger

## Overview

**Activity-Logger** is a cross-platform desktop utility that tracks user activity — such as key inputs and active window changes — and logs this data for productivity insights or diagnostics.

It works silently in the background with a modern **system tray icon** interface, providing quick access to logs and the option to gracefully exit the app.

The project is built in Rust for performance and efficiency and supports **Windows**, **Linux**, and **macOS** platforms with platform-specific integrations.

---

## Features

1. **Keylogging**
   Captures and logs user key inputs with timestamps.

2. **Active Window Tracking**
   Records the currently focused window and logs its title and associated app.

3. **System Tray Icon** (<img src="assets/active_icon.ico" alt="Icon" width="15"/>)
   - Lightweight tray icon runs in background
   - Right-click menu with:
     - **Show Logs**
     - **Open Config**
     - **Clear Logs**
     - **Quit**
   - Seamless background operation — no taskbar clutter.

4. **Configurable Logging**
   - `config.json` allows you to customize file names, storage directory, and inactivity timeout.

5. **Hidden Console on Release Build (Windows)**
   - In **debug mode**, a terminal window is shown for development output.
   - In **release mode**, the app runs in the background silently with **no console window**.

6. **Cross-Platform Support**
   Windows, Linux, and macOS support, with native APIs for each.

---

## Installation

### 1. Prerequisites

- **Rust Toolchain**:
  Install from [https://rust-lang.org/tools/install](https://rust-lang.org/tools/install)

- **System Dependencies**:
  - **Linux**:
    ```bash
    sudo apt install xdotool libx11-dev libappindicator3-dev
    ```
  - **Windows/macOS**: No additional dependencies needed.

---
2. **Clone the Repository**
   ```bash
   git clone https://github.com/CrimsonDevil333333/Activity-Logger
   cd Activity-Logger
   ```

3. **Build the Project**
   ```bash
   cargo build --release
   ```

4. **Run the Application**
   ```bash
   cargo run
   ```

---

## Configuration

The application uses a `config.json` file to specify logging preferences. Below is an example configuration file:

```json
{
  "key_log_file": "keys.log",
  "window_log_file": "windows.log",
  "log_dir": "c://logs",
  "inactivity_timeout_secs": 5
}
```

### Fields:
- `key_log_file`: File name for key input logs.
- `window_log_file`: File name for active window logs.
- `log_dir`: Directory where logs will be stored. Use `"temp"` to store in the system's temporary directory.
- `inactivity_timeout_secs`: Time (in seconds) to wait before flushing buffered key inputs due to inactivity.

Place the `config.json` file in the same directory as the executable.

---

## Log Output Examples

### Key Input Logs
Each line contains a timestamp and the recorded key inputs:

```
[2025-05-19 13:22:00] [App: Fleet.exe | Title: Terminal — Activity-Logger] Input: Hello World
[2025-05-19 13:22:14] [App: Fleet.exe | Title: Terminal — Activity-Logger] Input: This is test
[2025-05-19 13:22:17] [App: Fleet.exe | Title: Terminal — Activity-Logger] Input: Rust is cool

```

### Active Window Logs
Each line contains a timestamp and the title of the active window:

```
[2025-05-19 12:30:24] Active Window: App: Fleet.exe | Title: Terminal — Activity-Logger
[2025-05-19 13:18:48] Active Window: App: Explorer.EXE | Title: release - File Explorer
```

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit changes: `git commit -m "Add feature-name"`.
4. Push to your branch: `git push origin feature-name`.
5. Submit a pull request.

---

## Acknowledgments

- The Rust community for providing excellent tools and libraries.