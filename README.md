# TUF Control Center (Linux)

> A polished, Linux-native, open-source G-Helper alternative for Ubuntu on ASUS TUF and ROG gaming laptops.

TUF Control Center provides centralized hardware control over ASUS gaming laptops running Ubuntu Linux, with first-class support for the **ASUS TUF Gaming A15 FA506NC** and extensible support for other ASUS TUF and ROG models.

![TUF Control Center Banner](data/icons/tuf-control-center.svg)

---

## Highlights

- **Native GTK4 + Libadwaita UI**: Lightweight, responsive desktop application matching GNOME design guidelines and dark/light themes.
- **Hardware Telemetry**: Real-time CPU, GPU, memory, fan RPM, battery health, and power metrics polled efficiently with zero UI lag.
- **Performance Profiles**: Instant switching between Silent, Balanced, Turbo, and Custom modes mapped directly to `asus-nb-wmi` thermal throttle policies, Linux ACPI platform profiles, and `power-profiles-daemon`.
- **GPU Management**: Complete visibility and switching between Hybrid (Optimus/PRIME offloading), Integrated (dGPU powered down for battery life), and Dedicated graphics modes, integrating with `supergfxctl` and Linux runtime PM.
- **Battery Health Control**: Hardware charge thresholds (60%, 80%, 100%) applied directly via Linux ACPI power supply interfaces and persisted safely across reboots.
- **Fan Monitoring & Thermal Curves**: Real-time dual-fan monitoring (CPU and GPU RPM) and hardware-validated fan curve editing where supported by firmware.
- **Keyboard RGB & Lighting**: Single-zone and multi-zone TUF/ROG keyboard backlight brightness and animation modes (Static, Breathing, Strobing, Color Cycle) via ASUS HID and sysfs.
- **Display Configuration**: Quick refresh rate switching (60Hz / 144Hz) and brightness management via GNOME Mutter DisplayConfig.
- **Privileged Hardware Daemon**: Least-privilege D-Bus architecture utilizing Polkit for secure, audited hardware interactions without running the GUI as root.
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
