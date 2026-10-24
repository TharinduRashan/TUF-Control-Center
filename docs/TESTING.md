# Testing Strategy & Verification Plan

`TUF Control Center` includes automated test suites, hardware-independent mock testing, simulation mode, and a manual hardware verification procedure specifically targeted at the **ASUS TUF Gaming A15 FA506NC**.

---

## 1. Automated Testing Architecture

The codebase separates hardware I/O behind the `HardwareBackend` trait in `tuf-core`:

```rust
pub trait HardwareBackend: Send + Sync {
    fn detect_capabilities(&self) -> HardwareCapabilities;
    fn read_telemetry(&self) -> Result<SystemTelemetry, HardwareError>;
    fn set_performance_mode(&self, mode: PerformanceMode) -> Result<(), HardwareError>;
    fn set_battery_limit(&self, limit: BatteryLimit) -> Result<(), HardwareError>;
    fn set_keyboard_rgb(&self, config: &RgbConfig) -> Result<(), HardwareError>;
    fn set_keyboard_brightness(&self, level: u8) -> Result<(), HardwareError>;
    fn get_gpu_mode(&self) -> Result<GpuMode, HardwareError>;
    fn set_gpu_mode(&self, mode: GpuMode) -> Result<(), HardwareError>;
}
```

### Implementations:
1. `LinuxHardwareBackend`: Interacts with real sysfs, hwmon, D-Bus, and NVML.
2. `MockHardwareBackend`: Emulates an ASUS TUF Gaming A15 FA506NC in memory, producing realistic synthetic sensor data and recording all state changes for assert verification.

---

## 2. Unit & Integration Test Suites

### Automated Test Coverage:
1. **Hardware Capability Detection**:
   - Tests detection against simulated DMI product names (`ASUS TUF Gaming A15 FA506NC`, `ROG Zephyrus G14`, generic desktop, unsupported vendor).
   - Validates that non-existent features (e.g., hardware MUX on FA506NC) are correctly marked as `Unsupported`.
2. **Config Validation & Migration**:
   - Tests reading, writing, and parsing versioned TOML configs.
   - Tests graceful fallback to defaults when configuration contains corrupted or out-of-bounds keys.
3. **Privilege & Input Validation**:
   - Tests that values outside valid bounds (e.g. battery limit `75`, mode `99`, invalid RGB colors) are rejected with clear error codes.
   - Tests that symlink or traversal attempts are blocked.
4. **Thermal & Fan Bounds**:
   - Tests that fan curve temperatures must be monotonically increasing.
   - Tests that zero-RPM fan settings at high temperatures are rejected by safety validators.
5. **GPU State Machine**:
   - Tests transitions between `Hybrid`, `Integrated`, and `Dedicated`.
   - Tests notification when an active OpenGL/Vulkan application is running during a requested transition.

---

## 3. Simulation Mode (`--simulate`)

When executed with `--simulate` or `TUF_SIMULATE=1`:
- Real hardware sysfs paths are not touched.
- An unmistakable banner **"SIMULATION MODE — Mock Hardware Active"** is displayed in the GUI header.
- Dynamic telemetry generators simulate fluctuating CPU temperature (45°C - 82°C), fan speeds (1800 - 4600 RPM), battery discharge curves, and GPU state transitions.
- Fully functional for UI testing, screenshot generation, and CI/CD pipelines on headless runners.

---

## 4. Manual Hardware Test Plan (ASUS TUF Gaming A15 FA506NC)

Execute on physical hardware running Ubuntu:

| Test ID | Test Case | Action | Expected Result | Pass/Fail Criteria |
| :--- | :--- | :--- | :--- | :--- |
| **TC-01** | Battery Limit 80% | In UI Battery card, select "80% Longevity". | Value written to `/sys/class/power_supply/BAT0/charge_control_end_threshold`. | Battery stops charging once 80% is reached. |
| **TC-02** | Battery Limit 60% | Select "60% Maximum Lifespan". | sysfs threshold changes to 60. | Charging halts at 60%. |
| **TC-03** | Mode: Silent | Select "Silent" profile. | `throttle_thermal_policy` reads `2`. Fans spin down below audible threshold; CPU TDP drops. | Acoustic level drops; fan RPM decreases. |
| **TC-04** | Mode: Turbo | Select "Turbo" profile. | `throttle_thermal_policy` reads `1`. Fans ramp up to maximum profile; CPU boost enabled. | Fan RPM increases to peak range. |
| **TC-05** | Dual Fan Telemetry | Observe CPU and GPU Fan RPM. | Both RPM readings update every second. | Matches `fan1_input` and `fan2_input` in hwmon. |
| **TC-06** | Keyboard RGB: Static | Pick Color `#FF0000` (Red), mode Static. | All keyboard backlights switch to steady red. | Uniform red illumination. |
| **TC-07** | Keyboard RGB: Breathing | Select Breathing mode, Normal speed. | Keyboard smoothly pulses brightness. | Breathing animation visible. |
| **TC-08** | Keyboard Brightness | Adjust slider from 0 to 3. | LEDs dim down to off (0) and max brightness (3). | sysfs brightness reads requested value. |
| **TC-09** | Display 144Hz <-> 60Hz | Switch refresh rate to 60Hz, then back to 144Hz. | GNOME Mutter applies mode change cleanly. | Zero artifacts, refresh verified in GNOME Display Settings. |
| **TC-10** | Service Failure Recovery | Kill `tuf-daemon` with SIGTERM. | System remains stable; EC firmware maintains safe thermal profile. | No thermal throttling crashes. |
