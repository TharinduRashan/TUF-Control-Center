#!/usr/bin/env bash
set -euo pipefail

PACKAGE_NAME="tuf-control-center"
VERSION="1.0.0"
ARCH="amd64"
DIST_DIR="$(pwd)/dist"
BUILD_DIR="/tmp/tuf-debian-build"

echo "=== Building TUF Control Center Debian Package (${VERSION}) ==="

# 1. Compile release binaries
echo "-> Compiling release binaries via Cargo..."
cargo build --release

# 2. Prepare staging directory
echo "-> Creating staging filesystem layout..."
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}/DEBIAN"
mkdir -p "${BUILD_DIR}/usr/bin"
mkdir -p "${BUILD_DIR}/usr/lib/systemd/system"
mkdir -p "${BUILD_DIR}/usr/share/applications"
mkdir -p "${BUILD_DIR}/usr/share/icons/hicolor/scalable/apps"
mkdir -p "${BUILD_DIR}/usr/share/dbus-1/system.d"
mkdir -p "${BUILD_DIR}/usr/share/polkit-1/actions"

# 3. Copy binaries
echo "-> Copying executable binaries..."
if [ -f "target/release/tuf-control-center" ]; then
    cp "target/release/tuf-control-center" "${BUILD_DIR}/usr/bin/"
elif [ -f "target/release/tuf-gui" ]; then
    cp "target/release/tuf-gui" "${BUILD_DIR}/usr/bin/tuf-control-center"
fi
cp "target/release/tuf-daemon" "${BUILD_DIR}/usr/bin/"
cp "target/release/tuf-cli" "${BUILD_DIR}/usr/bin/"
chmod 755 "${BUILD_DIR}/usr/bin/"*

# 4. Copy system integration assets
echo "-> Installing system services, D-Bus, and desktop assets..."
cp "data/org.tuf.ControlCenter.service" "${BUILD_DIR}/usr/lib/systemd/system/tuf-daemon.service"
cp "data/org.tuf.ControlCenter.desktop" "${BUILD_DIR}/usr/share/applications/"
cp "data/icons/tuf-control-center.svg" "${BUILD_DIR}/usr/share/icons/hicolor/scalable/apps/"
cp "data/org.tuf.ControlCenter.conf" "${BUILD_DIR}/usr/share/dbus-1/system.d/"
cp "data/org.tuf.controlcenter.policy" "${BUILD_DIR}/usr/share/polkit-1/actions/"

# 5. Create Debian control file
echo "-> Generating DEBIAN/control..."
cat << 'EOF' > "${BUILD_DIR}/DEBIAN/control"
Package: tuf-control-center
Version: 1.0.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: TUF Control Center Contributors <https://github.com/tuf-control-center/tuf-control-center>
Depends: libgtk-4-1 (>= 4.6), libadwaita-1-0 (>= 1.0), libdbus-1-3 (>= 1.12), polkitd | policykit-1, systemd
Description: A Linux-native G-Helper clone for ASUS TUF and ROG gaming laptops
 TUF Control Center provides native hardware controls for ASUS gaming laptops,
 including operating profiles (Silent/Balanced/Turbo), dual-fan telemetry,
 battery charge thresholds (60%/80%/100%), GPU mode management, keyboard RGB,
 and display refresh rates. Designed for Ubuntu with GNOME integration.
EOF
chmod 755 "${BUILD_DIR}/DEBIAN"
chmod 644 "${BUILD_DIR}/DEBIAN/control"

# 6. Create post-install script (enable daemon, reload services)
cat << 'EOF' > "${BUILD_DIR}/DEBIAN/postinst"
#!/bin/sh
set -e

if [ "$1" = "configure" ]; then
    # Reload and start systemd service if systemd is running as PID 1
    if [ -d /run/systemd/system ]; then
        echo "Reloading systemd and activating tuf-daemon.service..."
        systemctl daemon-reload || true
        systemctl enable tuf-daemon.service || true
        systemctl restart tuf-daemon.service || true
    fi
    
    # Update icon and desktop cache
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
    fi
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database -q /usr/share/applications || true
    fi
fi
exit 0
EOF
chmod 755 "${BUILD_DIR}/DEBIAN/postinst"

# 7. Create pre-remove script (stop and disable daemon)
cat << 'EOF' > "${BUILD_DIR}/DEBIAN/prerm"
#!/bin/sh
set -e

if [ "$1" = "remove" ] || [ "$1" = "deconfigure" ]; then
    if [ -d /run/systemd/system ]; then
        echo "Stopping and disabling tuf-daemon.service..."
        systemctl stop tuf-daemon.service || true
        systemctl disable tuf-daemon.service || true
    fi
fi
exit 0
EOF
chmod 755 "${BUILD_DIR}/DEBIAN/prerm"

# 8. Create post-remove script (cleanup on purge)
cat << 'EOF' > "${BUILD_DIR}/DEBIAN/postrm"
#!/bin/sh
set -e

if [ "$1" = "purge" ] || [ "$1" = "remove" ]; then
    if [ -d /run/systemd/system ]; then
        systemctl daemon-reload || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
    fi
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database -q /usr/share/applications || true
    fi
fi
exit 0
EOF
chmod 755 "${BUILD_DIR}/DEBIAN/postrm"

# 9. Build Debian package
mkdir -p "${DIST_DIR}"
DEB_FILE="${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
echo "-> Building .deb package with dpkg-deb..."
dpkg-deb --build --root-owner-group "${BUILD_DIR}" "${DEB_FILE}"

echo "=== Successfully built: ${DEB_FILE} ==="
ls -lh "${DEB_FILE}"
