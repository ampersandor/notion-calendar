#!/bin/bash

export DISPLAY=:0
export XAUTHORITY=/home/pi/.Xauthority

# Install sxiv if not already installed
if ! command -v sxiv &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y sxiv
fi

# Wait for X server and services to start
sleep 8

# Ensure proper permissions
xhost +local:

# Kill any existing processes
killall unclutter
killall feh
killall notion_calendar

# Hide mouse cursor
unclutter -idle 0.1 -root &

# Change to calendar directory
cd /home/pi/notion-calendar/notion_calendar

# Start calendar program
./target/release/notion_calendar &

# Wait for the first image to be generated
sleep 5

# Display image with auto-reload
feh --hide-pointer --fullscreen --scale-down --zoom fill --reload 600 calendar.png