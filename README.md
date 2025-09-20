# Dynamic Power Profile

A small daemon that dynamically adjusts the power profile based on the system's power status. On battery power, it will switch to a lower power profile to conserve battery life. On AC power, it will switch to a higher power profile to maximize performance.

## Installation

```
cargo install dynamic-power-profile
```

## Systemd

Create a systemd service file in `~/.config/systemd/user/dynamic-power-profile.service`:

```
[Unit]
Description=Dynamic Power Profile Service
After=network.target

[Service]
ExecStart=/home/myuser/.cargo/bin/dynamic-power-profile
WorkingDirectory=/home/myuser/
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
```

and then enable and start the service:

```bash
systemctl --user enable dynamic-power-profile.service
systemctl --user start dynamic-power-profile.service
```
