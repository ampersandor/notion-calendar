#!/bin/bash

export DBUS_SESSION_BUS_ADDRESS=unix:path=/run/dbus/system_bus_socket
export DISPLAY=:0
export XAUTHORITY=/home/pi/.Xauthority

KIOSK_URL="https://portfolio.ampersandor.app"

# Wait for services to come online.
sleep 8

# Ensure proper permissions
xhost +local:

echo 'Hiding the mouse cursor...'
unclutter -idle 0.1 -root &

echo 'Starting Chromium...'
/usr/bin/chromium-browser \
  --noerrdialogs \
  --disable-infobars \
  --kiosk \
  --disable-gpu \
  --no-sandbox \
  --disable-features=TranslateUI \
  --disable-session-crashed-bubble \
  --disable-popup-blocking \
  --app=$KIOSK_URL

# 이미지 뷰어 실행 (예: feh)
feh --reload 1 --fullscreen calendar.png &

# 캘린더 프로그램 실행
cd notion_calendar
cargo run