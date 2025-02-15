```bash
sudo apt install unclutter
```

```bash
sudo apt install chromium-browser
```

```bash
cp kiosk.sh /home/pi/kiosk/kiosk.sh
sudo cp kiosk.service /lib/systemd/system/kiosk.service
sudo systemctl enable kiosk.service
sudo systemctl start kiosk
```

