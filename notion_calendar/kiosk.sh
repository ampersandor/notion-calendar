#!/bin/bash

export DISPLAY=:0
export XAUTHORITY=/home/pi/.Xauthority

# Wait for X server and services to start
sleep 8

# Ensure proper permissions
xhost +local:

# Hide mouse cursor
unclutter -idle 0.1 -root &

# Start window manager (optional but recommended)
openbox &

# Build calendar if needed
cd /home/pi/notion-calendar/notion_calendar
cargo build --release

# Start calendar program
./target/release/notion_calendar &

# Start image viewer (10분마다 새로고침)
feh --hide-pointer --fullscreen --auto-reload --reload 600 calendar.png
