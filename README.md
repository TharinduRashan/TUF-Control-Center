# TUF Control Center (Linux)

> A polished, Linux-native, open-source G-Helper alternative for Ubuntu on ASUS TUF and ROG gaming laptops.

TUF Control Center provides centralized hardware control over ASUS gaming laptops running Ubuntu Linux, with first-class support for the **ASUS TUF Gaming A15 FA506NC** and extensible support for other ASUS TUF and ROG models.

![TUF Control Center Banner](data/icons/tuf-control-center.svg)

---

## Feature Matrix & Hardware Support (FA506NC)

| Feature | Support Status | Backend / Kernel Driver | Notes |
| :--- | :---: | :--- | :--- |
| **Operating Profiles** | ✅ Supported | `asus-nb-wmi` / platform_profile | Silent (Quiet), Balanced, Turbo (Performance) |
| **Dual Fan Telemetry** | ✅ Supported | `asus-nb-wmi` / `hwmon` | Real-time CPU & GPU RPM polling |
| **Battery Charge Limit**| ✅ Supported | `asus-nb-wmi` / `charge_control_end_threshold` | 60%, 80%, 100% thresholds persisted |
| **Keyboard RGB Lighting**| ✅ Supported | ASUS HID raw report (`/dev/hidraw*`) | Static, Breathing, Strobing, Color Cycle |
| **Backlight Brightness**| ✅ Supported | `asus::kbd_backlight` sysfs | 0 (Off) to 3 (Max) levels |
| **Display Refresh Rate**| ✅ Supported | GNOME Mutter / `xrandr` / `wlr-randr` | 60Hz / 144Hz quick-switch |
| **GPU Mode: Hybrid** | ✅ Supported | Linux PRIME offload / `supergfxctl` | Default dynamic power-saving |
| **GPU Mode: Integrated**| ✅ Supported | Linux Runtime D3cold / `supergfxctl` | Powers off dGPU for maximum battery |
| **GPU Mode: Dedicated** | ❌ Unsupported | Hardware Limitation | FA506NC lacks a hardware MUX switch |

---

## Highlights

- **Native GTK4 + Libadwaita UI**: Lightweight, responsive desktop application matching GNOME design guidelines and dark/light themes.
- **Hardware Telemetry**: Real-time CPU, GPU, memory, fan RPM, battery health, and power metrics polled efficiently with zero UI lag.
- **Performance Profiles**: Instant switching between Silent, Balanced, Turbo, and Custom modes mapped directly to `asus-nb-wmi` thermal throttle policies, Linux ACPI platform profiles, and `power-profiles-daemon`.
- **GPU Management**: Complete visibility and switching between Hybrid (Optimus/PRIME offloading) and Integrated (dGPU powered down for battery life) modes via `supergfxctl`.
- **Battery Health Control**: Hardware charge thresholds (60%, 80%, 100%) applied directly via Linux ACPI power supply interfaces and persisted safely across reboots.
- **Fan Monitoring & Thermal Curves**: Real-time dual-fan monitoring (CPU and GPU RPM) with fail-safe embedded controller fallback.
- **Keyboard RGB & Lighting**: Single-zone TUF keyboard backlight brightness and animation modes (Static, Breathing, Strobing, Color Cycle) via ASUS HID and sysfs.
- **Display Configuration**: Quick refresh rate switching (60Hz / 144Hz) via Wayland & X11 display servers.
- **Privileged Hardware Daemon**: Strict least-privilege D-Bus architecture utilizing PolicyKit (`pkcheck`) with fail-closed security for safe hardware control.
- **Hardware Simulation Mode**: Built-in mock mode allowing full UI testing and development on any Linux machine or VM without ASUS hardware.

---

## Target Hardware & Environment

- **Primary Target**: ASUS TUF Gaming A15 (FA506NC)
  - CPU: AMD Ryzen 5 7535HS (Rembrandt-R Zen 3+, 6C/12T)
  - dGPU: NVIDIA GeForce RTX 3050 Laptop GPU (Ampere GA107)
  - iGPU: AMD Radeon 660M (RDNA 2)
  - Display: 15.6" FHD 144Hz IPS panel
  - Keyboard: Single-zone ASUS TUF RGB Backlight
- **Operating System**: Ubuntu 24.04 LTS / 26.04 LTS (GNOME Wayland/X11)
- **Audio Stack**: Preserves PipeWire & WirePlumber compatibility

---

## Quick Installation (Ubuntu 24.04 / 26.04)

### Install via Debian Package (.deb)
Download the latest release package from the [Releases](https://github.com/tuf-control-center/tuf-control-center/releases) page and install:

```bash
sudo apt update
sudo apt install -y ./tuf-control-center_1.0.0_amd64.deb
```

The background service (`tuf-daemon.service`) will automatically start and enable itself. You can launch **TUF Control Center** from your application launcher or by running:
```bash
tuf-control-center
```

---

## CLI Usage

TUF Control Center includes a comprehensive command-line tool `tuf-cli`:

```bash
# View real-time hardware status
tuf-cli status

# Set operating mode
tuf-cli profile balanced # or silent, turbo

# Set battery charge threshold
tuf-cli battery-limit 80

# Configure RGB lighting
tuf-cli rgb --mode static --color "#FF8000" --speed normal
```

---

## Documentation

- [Feature Compatibility Matrix](docs/FEATURE_MATRIX.md)
- [Architecture & Design](docs/ARCHITECTURE.md)
- [Hardware Support & Quirks](docs/HARDWARE_SUPPORT.md)
- [Security & Privileged Operations](docs/SECURITY.md)
- [Testing & Verification](docs/TESTING.md)
- [Installation & Packaging](docs/INSTALLATION.md)
- [Development Tasks (TODO)](TODO.md)
- [Agent Guidance](AGENTS.md)

---

## License

GPL-3.0-or-later
