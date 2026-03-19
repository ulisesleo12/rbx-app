#!/usr/bin/env bash

# .ssh/config
#
# Host aker-service
#     HostName 104.248.55.109
#     KeepAlive yes
#     ServerAliveInterval 60
#     Port 22
#     User aker
#     IdentityFile ~/.ssh/id_rsa

trunk build --release && \
scp -r dist/* robox-netvm:/var/www/app.roboxmaker.network/html
