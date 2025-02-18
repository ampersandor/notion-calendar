#!/bin/bash

export DISPLAY=:0
export XAUTHORITY=/home/pi/.Xauthority

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

# Display and refresh image
while true; do
    if [ -f "calendar.png" ]; then
        feh --hide-pointer --fullscreen calendar.png
        sleep 600  # 10분 대기
        killall feh  # 기존 feh 프로세스 종료
    else
        sleep 1
    fi
done