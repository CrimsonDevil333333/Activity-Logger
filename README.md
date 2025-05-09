# Activity-Logger

## Overview

**Activity-Logger** is a cross-platform application designed to track user activity, including key inputs and active window changes. It logs this data into specified files for analysis, making it a useful tool for productivity tracking, debugging, or other monitoring purposes.

The project supports Windows, Linux, and macOS platforms and leverages Rust's concurrency and performance capabilities to efficiently monitor and log user activity in near real-time.

---

## Features

1. **Keylogging**
   Captures and logs user key inputs along with timestamps.

2. **Active Window Tracking**
   Tracks the currently focused application window and logs its title with timestamps.

3. **Configurable Logging**
   - Adjustable file paths for logs (`key_log_file`, `window_log_file`).
   - Configurable inactivity timeout for flushing buffered key inputs.

4. **Cross-Platform Support**
   Platform-specific implementations for Windows, Linux, and macOS.

5. **Concurrency**
   Utilizes Rust's multithreading capabilities to efficiently handle logging tasks.

---

## Installation

1. **Prerequisites**
   - Rust toolchain installed. Follow the [official instructions](https://www.rust-lang.org/tools/install).
   - System dependencies:
     - **Linux**: `xdotool` and `libx11-dev` (can be installed via `sudo apt-get install xdotool libx11-dev`).
     - **Windows/MacOS**: No additional dependencies required.

2. **Clone the Repository**
   ```bash
   git clone https://github.com/your-username/Activity-Logger.git
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
  "log_dir": "c://logs" or  "./logs/" depending on OS,
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
[2025-05-09 10:15:30] Input: hello world
[2025-05-09 10:15:35] Input: rust is awesome
```

### Active Window Logs
Each line contains a timestamp and the title of the active window:

```
[2025-05-09 10:15:30] Active Window: Visual Studio Code
[2025-05-09 10:15:35] Active Window: Firefox
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