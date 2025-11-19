# Activity Logger - Distribution Package

This is a portable distribution of Activity Logger. No installation required!

## Quick Start

### Windows
1. Extract the ZIP file to any location
2. Double-click `run.bat` to start the application
3. The dashboard will be available at http://localhost:8080
4. Look for the Activity Logger icon in your system tray

### Linux
1. Extract the archive to any location
2. Run `./run.sh` from the terminal
3. The dashboard will be available at http://localhost:8080
4. Look for the Activity Logger icon in your system tray

## Configuration

Edit `config.json` to customize:
- Log file locations
- Screenshot settings
- Server port
- Inactivity timeout
- Notifications

## Features

- **Key Logging**: Track keyboard inputs with timestamps
- **Window Tracking**: Monitor active window changes
- **Screenshot Capture**: Automatic screenshot capture at intervals
- **Web Dashboard**: Modern web interface at http://localhost:8080
  - View logs and screenshots
  - Search and filter logs
  - Clear data
  - View statistics
  - Edit configuration
- **System Tray**: Quick access to all features

## Dashboard Features

- **Statistics Overview**: View total logs, screenshots, and disk usage
- **Search & Filter**: Real-time search across all logs
- **Clear Options**: Delete logs and screenshots with confirmation
- **Auto-refresh**: Configurable refresh intervals (5s, 10s, 30s)
- **Config Editor**: Edit settings directly from the web interface

## System Requirements

### Windows
- Windows 10 or later
- No additional dependencies required

### Linux
- X11 display server
- libx11, libappindicator3, libdbus-1

## Logs Location

By default, logs are saved to:
- Windows: `%TEMP%\activity_logger\`
- Linux: `/tmp/activity_logger/`

You can change this in `config.json`.

## Support

For issues or questions, visit: https://github.com/CrimsonDevil333333/Activity-Logger

## License

MIT License - See the main repository for details.
