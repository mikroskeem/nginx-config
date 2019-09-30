#!/usr/bin/env bash
set -e

echo ">>> Creating /var/www/certbot"
install -d -m 0755 /var/www/certbot

echo ">>> Installing certbot nginx reload addition"
install -D -m 0644 /dev/stdin /etc/systemd/system/certbot.service.d/nginx-reload.conf <<EOF
[Service]
ExecStopPost=/bin/systemctl --no-block reload nginx.service
EOF

systemctl daemon-reload

echo ">>> Enabling and starting certbot timer"
systemctl enable --now certbot.timer

echo ""
echo ""
echo ">>> Done!"
