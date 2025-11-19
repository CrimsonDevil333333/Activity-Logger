#!/bin/bash
# Activity Logger - Linux Launcher
# This script launches the Activity Logger application

echo "Starting Activity Logger..."
echo ""
echo "Dashboard will be available at: http://localhost:8080"
echo "Check the system tray for the Activity Logger icon."
echo ""

./activity_logger &

echo "Activity Logger is now running in the background."
echo "You can close this terminal."
