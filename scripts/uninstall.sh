#!/usr/bin/env bash
set -euo pipefail

if [ "$(id -u)" -ne 0 ]; then
    echo "Error: uninstall.sh must be run with root privileges (sudo)." >&2
    exit 1
fi

echo "=== Uninstalling TUF Control Center ==="

# Stop and disable systemd service
if systemctl is-active --quiet tuf-daemon.service 2>/dev/null; then
    echo "-> Stopping tuf-daemon.service..."
    systemctl stop tuf-daemon.service || true
fi

if systemctl is-enabled --quiet tuf-daemon.service 2>/dev/null; then
    echo "-> Disabling tuf-daemon.service..."
    systemctl disable tuf-daemon.service || true
fi

# Remove installed system files
echo "-> Removing binaries..."
rm -f /usr/bin/tuf-control-center
rm -f /usr/bin/tuf-daemon
rm -f /usr/bin/tuf-cli

echo "-> Removing systemd service unit..."
rm -f /usr/lib/systemd/system/tuf-daemon.service
systemctl daemon-reload || true

echo "-> Removing D-Bus and PolicyKit configurations..."
rm -f /usr/share/dbus-1/system.d/org.tuf.ControlCenter.conf
rm -f /usr/share/polkit-1/actions/org.tuf.controlcenter.policy

echo "-> Removing desktop integration and icons..."
rm -f /usr/share/applications/org.tuf.ControlCenter.desktop
rm -f /usr/share/icons/hicolor/scalable/apps/tuf-control-center.svg

if which gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi
if which update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi

# Handle user data purge flag
if [[ "${1:-}" == "--purge-data" ]]; then
    echo "-> Purging user configuration files..."
    for home_dir in /home/*; do
        if [ -d "$home_dir/.config/tuf-control-center" ]; then
            rm -rf "$home_dir/.config/tuf-control-center"
            echo "   Removed config in $home_dir"
        fi
    done
    rm -rf /root/.config/tuf-control-center
fi

echo "=== TUF Control Center uninstalled successfully ==="
