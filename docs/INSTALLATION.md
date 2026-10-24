# Installation, Deployment & Distribution Guide

This guide covers building, installing, configuring, and uninstalling **TUF Control Center** on Ubuntu 24.04 LTS and 26.04 LTS.

---

## 1. System Requirements

- **Operating System**: Ubuntu 24.04 LTS or 26.04 LTS (GNOME desktop environment recommended)
- **Supported Kernel**: Linux 6.8+ (includes modern `asus-wmi` and `asus-nb-wmi` driver support)
- **Privileges**: Standard user for the GUI; `sudo` for initial package installation
- **Dependencies**:
  - `libgtk-4-1` (>= 4.12)
  - `libadwaita-1-0` (>= 1.5)
  - `libdbus-1-3` (>= 1.14)
  - `polkitd` / `policykit-1`
  - `systemd`

---

## 2. Quick Installation via Pre-Built Debian Package (.deb)

The easiest and cleanest method is installing the official Debian package:

```bash
# 1. Download or locate the .deb package
sudo apt update
sudo apt install ./tuf-control-center_1.0.0_amd64.deb

# 2. Verify that the system daemon is active
systemctl status tuf-daemon.service

# 3. Launch from your GNOME application launcher or terminal
tuf-control-center
```

The package automatically:
- Installs binaries to `/usr/bin/tuf-control-center`, `/usr/bin/tuf-daemon`, `/usr/bin/tuf-cli`
- Installs the systemd service to `/usr/lib/systemd/system/tuf-daemon.service`
- Installs Polkit action rules to `/usr/share/polkit-1/actions/org.tuf.controlcenter.policy`
- Installs D-Bus system bus configuration to `/usr/share/dbus-1/system.d/org.tuf.ControlCenter.conf`
- Installs desktop entry and high-resolution scalable icons
- Enables and starts `tuf-daemon.service` immediately

---

## 3. Building From Source

### 3.1 Install Build Dependencies
```bash
sudo apt update
sudo apt install -y cargo rustc build-essential pkg-config \
    libgtk-4-dev libadwaita-1-dev libdbus-1-dev libsystemd-dev
```

### 3.2 Compile Workspace
```bash
# Clone or navigate to the repository directory
cd tuf-control-center

# Build all workspace crates in release mode
cargo build --release
```

Compiled binaries will be located at:
- `target/release/tuf-control-center` (GUI application)
- `target/release/tuf-daemon` (Privileged D-Bus service)
- `target/release/tuf-cli` (Command-line utility)

### 3.3 Running Automated Tests
```bash
cargo test
```

### 3.4 Running in Simulation Mode (No Root & No ASUS Laptop Needed)
```bash
cargo run --bin tuf-control-center -- --simulate
```

---

## 4. Building the Debian (.deb) Package

A dedicated packaging script is provided in `scripts/build-deb.sh`:

```bash
chmod +x scripts/build-deb.sh
./scripts/build-deb.sh
```

This creates `dist/tuf-control-center_1.0.0_amd64.deb` complete with proper metadata, control scripts (`postinst`, `prerm`), and desktop assets.

---

## 5. Uninstallation

To cleanly remove TUF Control Center without leaving orphan system files:

### Via APT:
```bash
sudo apt remove --purge tuf-control-center
```

### Via Uninstall Script:
```bash
sudo ./scripts/uninstall.sh
```

The uninstaller stops and disables `tuf-daemon.service`, removes the D-Bus and Polkit definitions, deletes application binaries and icons, and preserves user configuration files in `~/.config/tuf-control-center` unless the `--purge-data` flag is explicitly passed.

---

## 6. Troubleshooting

### Problem: Battery charge limit does not apply
1. Check that the kernel driver `asus_nb_wmi` is loaded:
   ```bash
   lsmod | grep asus_nb_wmi
   ```
2. Verify that `/sys/class/power_supply/BAT0/charge_control_end_threshold` exists:
   ```bash
   cat /sys/class/power_supply/BAT0/charge_control_end_threshold
   ```
3. If the node is missing, ensure your kernel is 6.5 or newer.

### Problem: GPU Mode Switching requires reboot / logout
On laptops without a dedicated MUX switch (such as TUF Gaming A15 FA506NC), switching to Integrated (Eco) mode requires unloading the NVIDIA proprietary driver. If active applications (like Steam or a Web Browser hardware acceleration instance) hold open `/dev/nvidia*` file descriptors, close those applications before switching or restart your desktop session.
