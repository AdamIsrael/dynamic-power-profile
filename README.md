# Dynamic Power Profile

## Installation

TBD:
```
cargo install dynamic-power-profile
```

## Systemd

TBD:

```
[Unit]
Description=Dynamic Power Profile Service
After=network.target

[Service]
ExecStart=/usr/local/bin/dynamic-power-profile
Restart=on-failure
RestartSec=5s
User=myuser
Group=mygroup

[Install]
WantedBy=multi-user.target
```
