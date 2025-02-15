#!/bin/bash

export DISPLAY=:0
export XAUTHORITY=/home/pi/.Xauthority
export RUST_BACKTRACE=1
export RUST_LOG=debug  # 디버그 로그 활성화

# Wait for X server and services to start
sleep 8

# Build clock if needed
cd /home/pi/notion_clock
cargo build --release

# Start clock program
./target/release/notion_clock 2>&1 | tee clock.log  # 로그 파일 생성 