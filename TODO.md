# Project Tasks & Roadmap (TODO)

## Phase 1: Research & Architecture (Completed)
- [x] Inspect ASUS TUF Gaming A15 FA506NC hardware interfaces, ACPI, and G-Helper parameters.
- [x] Create feature compatibility matrix (`docs/FEATURE_MATRIX.md`).
- [x] Define system architecture and least-privilege D-Bus model (`docs/ARCHITECTURE.md`).
- [x] Document hardware support, quirks, and limitations (`docs/HARDWARE_SUPPORT.md`).
- [x] Write security specification and Polkit design (`docs/SECURITY.md`).
- [x] Create automated testing strategy and manual FA506NC test plan (`docs/TESTING.md`).
- [x] Write installation and packaging guide (`docs/INSTALLATION.md`).
- [x] Write agent guidelines (`AGENTS.md`).

## Phase 2: Hardware Abstraction & Core Engine (`crates/tuf-core`) (Completed)
- [x] Define shared domain models (`PerformanceMode`, `BatteryLimit`, `GpuMode`, `RgbConfig`, `SystemTelemetry`).
- [x] Implement `HardwareCapabilities` detector (DMI vendor/product, sysfs probe, MUX detection).
- [x] Implement Linux telemetry reader:
  - [x] CPU temperature (`k10temp`), frequencies, and core utilization (`/proc/stat`).
  - [x] GPU telemetry (NVIDIA NVML / runtime PM D3cold check).
  - [x] Memory utilization (`/proc/meminfo`).
  - [x] Battery telemetry (level, status, health, wattage, thresholds).
  - [x] Dual-fan RPM telemetry (`fan1_input`, `fan2_input`).
- [x] Implement robust TOML configuration manager with automatic migration and schema versioning.
- [x] Implement full in-memory `MockHardwareBackend` for simulation mode and CI tests.
- [x] Pass automated test suite (6/6 tests passing).

## Phase 3: Privileged Hardware Daemon (`crates/tuf-daemon`) (Completed)
- [x] Implement D-Bus system service `org.tuf.ControlCenter`.
- [x] Implement PolicyKit (`polkit-1`) authorization validation.
- [x] Implement atomic sysfs controller:
  - [x] `throttle_thermal_policy` (0, 1, 2) & `platform_profile`.
  - [x] `charge_control_end_threshold` (60, 80, 100).
  - [x] `energy_performance_preference`.
  - [x] Keyboard brightness sysfs controller.
- [x] Implement ASUS TUF Aura RGB USB HID controller (`/dev/hidraw*` protocol).
- [x] Implement fail-safe watchdog (restores safe firmware fan control if interrupted).

## Phase 4: Command-Line Interface (`crates/tuf-cli`) (Completed)
- [x] Implement CLI commands: `status`, `profile`, `battery-limit`, `gpu`, `rgb`, `brightness`.
- [x] JSON and formatted human-readable output modes.
- [x] Validate in simulation mode.

## Phase 5: Native Desktop User Interface (`crates/tuf-gui`) (Completed)
- [x] Initialize GTK4 + Libadwaita application with dark/light theme support.
- [x] Design compact G-Helper inspired dashboard:
  - [x] Header bar with laptop model name, AC status, and simulation indicator.
  - [x] Telemetry cards: CPU, GPU, Memory, Fans, Battery.
  - [x] Operating Mode selector (Silent, Balanced, Turbo).
  - [x] GPU mode selector with transition warnings and recovery path.
  - [x] Battery charge threshold selector (60%, 80%, 100%).
  - [x] Fan status and telemetry viewer (firmware-managed acoustic curves on FA506NC).
  - [x] Display panel controls (144Hz / 60Hz toggle, brightness slider).
  - [x] Keyboard RGB controls (lighting mode, color picker, brightness).
  - [x] System session toggles: Touchpad (GNOME GSettings), Mic Mute (PipeWire).
  - [x] Application settings & diagnostics reporting.
- [x] Integrate asynchronous telemetry background worker (adaptive polling).

## Phase 6: Automated Testing & Validation (Completed)
- [x] Unit tests for hardware capability detection.
- [x] Unit tests for telemetry parsing and edge cases.
- [x] Unit tests for configuration serialization and validation.
- [x] Unit tests for input bounds validation in privileged operations.
- [x] Verification in `--simulate` mode.

## Phase 7: Packaging & Deployment (Completed)
- [x] Create systemd service unit (`tuf-daemon.service`).
- [x] Create D-Bus system bus configuration (`org.tuf.ControlCenter.conf`).
- [x] Create PolicyKit action file (`org.tuf.controlcenter.policy`).
- [x] Create FreeDesktop desktop entry (`org.tuf.ControlCenter.desktop`) and SVG icons.
- [x] Create Debian package builder script (`scripts/build-deb.sh`).
- [x] Create clean uninstaller script (`scripts/uninstall.sh`).
