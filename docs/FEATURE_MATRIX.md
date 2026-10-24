# Feature Compatibility Matrix

This document provides a comprehensive comparison between features in the original Windows G-Helper application and their Linux implementation in **TUF Control Center**, specifically tracking hardware requirements, Linux interfaces, privilege level, support status, and validation methods.

| G-Helper Feature | Linux Implementation Mechanism | Hardware Requirements | Required Privileges | Support Status (FA506NC) | Validation Method |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Operating Modes (Silent / Balanced / Turbo)** | `/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy` & `platform_profile` & `power-profiles-daemon` | ASUS WMI / ACPI platform profile support | Polkit / Privileged Daemon | **Supported** | Write policy index (0=Balanced, 1=Turbo, 2=Silent); verify read-back from sysfs and D-Bus. |
| **CPU Energy Performance Preference (EPP)** | `/sys/devices/system/cpu/cpufreq/policy*/energy_performance_preference` (`amd-pstate-epp`) | AMD Ryzen Zen 3+ (`amd-pstate=active`) | Privileged Daemon | **Supported** | Verify policies match `power`, `balance_power`, `balance_performance`, `performance`. |
| **GPU Mode: Hybrid (Optimus/PRIME)** | Standard NVIDIA PRIME offload (`__NV_PRIME_RENDER_OFFLOAD=1`) with runtime D3 cold power management | AMD iGPU + NVIDIA dGPU | Unprivileged for apps; privileged for boot/driver mode | **Supported** | Run `glxinfo` / `vulkaninfo` with offload variables; verify dGPU sleeps in runtime PM. |
| **GPU Mode: Integrated (Eco)** | `supergfxctl` (`Dedicated`, `Hybrid`, `Integrated`) or dGPU PCI remove/power cut via ACPI `_PS3`/sysfs | Discrete GPU power cutoff | Privileged Daemon / `supergfxctl` D-Bus | **Supported** | Verify NVIDIA device `/sys/bus/pci/devices/0000:01:00.0/power_state` is D3cold and driver unbinds safely. |
| **GPU Mode: Ultimate (Dedicated MUX)** | Hardware multiplexer (`_DSM` / ASUS MUX WMI `0x00090020`) | Physical display MUX chip | Privileged Daemon | **Unsupported on FA506NC** (Mux flag returns -65536) | Safely detect unsupported status and display clear badge in UI indicating no hardware MUX. |
| **Battery Charge Limit (60%, 80%, 100%)** | `/sys/class/power_supply/BAT0/charge_control_end_threshold` | ASUS ACPI Battery threshold support | Privileged Daemon | **Supported** | Write threshold percentage (60, 80, 100); verify battery ceases charging past limit. |
| **Dual Fan RPM Telemetry** | `/sys/devices/platform/asus-nb-wmi/hwmon/hwmon*/fan1_input` (CPU) and `fan2_input` (GPU) | ASUS WMI hwmon driver | Unprivileged (sysfs read) | **Supported** | Periodic non-blocking read of fan RPM values. |
| **Custom Fan Curves** | `/sys/devices/platform/asus-nb-wmi/hwmon/hwmon*/pwm1_enable` or ASUS WMI curve points | BIOS support for custom fan table | Privileged Daemon | **Partial / Firmware Only on FA506NC** | TUF models tie fan curves to thermal throttle policy in BIOS; report firmware curve status cleanly. |
| **CPU Telemetry (Temp, Load, Freq)** | AMD `k10temp` / `zenpower` hwmon (`Tctl`, `Tdie`) & `/proc/stat` & `/proc/cpuinfo` | AMD Ryzen processor | Unprivileged | **Supported** | Read `/sys/class/hwmon/hwmon*/temp1_input`; calculate diffs from `/proc/stat`. |
| **GPU Telemetry (Temp, Load, Power, VRAM)** | NVIDIA NVML (`libnvidia-ml.so` / `nvidia-smi`) | NVIDIA proprietary driver or open-kernel modules | Unprivileged | **Supported** | Poll NVML when dGPU is awake; avoid waking sleeping dGPU in Hybrid mode. |
| **Keyboard RGB Brightness** | `/sys/class/leds/asus::kbd_backlight/brightness` | ASUS WMI / HID keyboard LED | Privileged Daemon / udev rule | **Supported** | Read/write integer (0 to 3) to sysfs brightness node. |
| **Keyboard RGB Lighting Modes** | ASUS TUF HID protocol via `/dev/hidraw*` or `/sys/class/leds/asus::kbd_backlight/kbd_rgb_mode` | ASUS TUF keyboard controller (0x0B05:0x19b6/etc.) | Privileged Daemon | **Supported** | Send 16-byte Aura TUF packet (Static, Breathing, Strobing, Color Cycle, Speed, RGB). |
| **Display Refresh Rate (60Hz / 144Hz)** | GNOME Mutter D-Bus `org.gnome.Mutter.DisplayConfig` or `libdrm` | Variable refresh display panel | Unprivileged (User session D-Bus) | **Supported** | Query modes via Mutter D-Bus, apply selected frequency seamlessly without tearing. |
| **Display Brightness** | `/sys/class/backlight/amdgpu_bl*/brightness` or `org.gnome.SettingsDaemon.Power.Screen` | Internal panel backlight controller | Unprivileged via D-Bus / udev | **Supported** | Read and adjust screen brightness smoothly. |
| **Fn-Lock Toggle** | `/sys/devices/platform/asus-nb-wmi/fn_lock` | ASUS WMI Fn-lock method | Privileged Daemon | **Unsupported on FA506NC** | Confirmed unsupported on FA506NC (`FnLock Support: -65536`). Badge as unsupported. |
| **Touchpad Toggle** | GNOME Settings `org.gnome.desktop.peripherals.touchpad` / `libinput` | Standard touchpad input | Unprivileged | **Supported** | Read/write `send-events` key in `org.gnome.desktop.peripherals.touchpad`. |
| **Microphone Mute Toggle** | PipeWire / WirePlumber D-Bus & ALSA mixer | Linux audio stack | Unprivileged | **Supported** | Read and toggle default capture source mute via WirePlumber / ALSA. |
| **Status Tray Indicator** | Ayatana AppIndicator3 / StatusNotifierItem / GNOME AppIndicator | GNOME Shell with AppIndicator support | Unprivileged | **Supported** | Provide tray menu with fallback for vanilla GNOME Shell environments. |
| **Audit Logging & Recovery** | Local systemd journal & `/var/log/tuf-control-center.log` | Linux filesystem | Unprivileged / Service logger | **Supported** | Structured logging of all hardware state changes and fail-safes. |

---

## Detailed Notes on Hardware Quirks

1. **MUX Switch Absence**:
   The FA506NC motherboard does not route the discrete GPU outputs directly through a hardware multiplexer to the internal eDP display. All internal rendering flows through the AMD Radeon 660M iGPU. The NVIDIA RTX 3050 operates in Hybrid (PRIME offload) mode or can be powered off (Eco/Integrated). The application accurately displays this hardware limitation to prevent confusion.

2. **Zero-Power Sleeping dGPU (RTD3)**:
   In Hybrid mode, waking the NVIDIA GPU solely to poll its temperature wastes battery power. TUF Control Center respects PCI runtime power management: if the GPU is in `suspended` / `D3cold` state, the UI displays `GPU Sleeping (0W)` rather than querying NVML and inadvertently waking the card.

3. **Battery Charge Persistence**:
   The ASUS battery threshold register in the embedded controller (EC) is written via `/sys/class/power_supply/BAT0/charge_control_end_threshold`. When set, the firmware stops charging upon reaching the target percentage. To guarantee persistence across power cycles and kernel updates, the TUF Control Center system service restores the user's selected threshold at system boot.
