# Hardware Support & Quirks

This document details hardware behavior, kernel driver requirements, known limitations, and device-specific quirks for ASUS TUF and ROG laptops under Ubuntu Linux, centered on the **ASUS TUF Gaming A15 FA506NC**.

---

## 1. ASUS TUF Gaming A15 FA506NC Profile

### Hardware Specifications
- **Model**: `ASUS TUF Gaming A15 FA506NC_FA506NC`
- **CPU**: AMD Ryzen 5 7535HS (6 cores, 12 threads, base 3.3 GHz, boost up to 4.55 GHz, Rembrandt-R)
- **iGPU**: AMD Radeon 660M (RDNA 2, 6 CUs)
- **dGPU**: NVIDIA GeForce RTX 3050 Laptop GPU (Ampere GA107, 4GB/6GB GDDR6)
- **Display**: 15.6" Full HD (1920x1080), 144Hz IPS panel (AUO / Innolux eDP)
- **Embedded Controller**: ASUS ACPI / WMI via `asus-nb-wmi`
- **Keyboard**: Single-zone RGB keyboard backlight with Aura effects

---

## 2. Kernel & Driver Interfaces

### 2.1 Thermal Throttle Policy & Performance Modes
On ASUS laptops, thermal profiles are managed directly by firmware in the Embedded Controller (EC).
The kernel module `asus-nb-wmi` exposes this at:
```text
/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy
```
Values:
- `0`: Balanced / Default (standard power envelope and fan curve)
- `1`: Turbo / Overboost (higher TDP ceiling, aggressive fan speed, boosted clocks)
- `2`: Silent (reduced fan noise, capped acoustic target, lowered CPU/GPU power limits)

Additionally, the ACPI platform profile is exposed at:
```text
/sys/firmware/acpi/platform_profile
```
Values: `quiet`, `balanced`, `performance`.

### 2.2 Battery Health & Charge Limit
The embedded controller retains the maximum charge percentage. Writing to:
```text
/sys/class/power_supply/BAT0/charge_control_end_threshold
```
Supported thresholds:
- `60`: Maximum lifespan for stationary/plugged-in operation.
- `80`: Balanced longevity and battery usability (recommended default for desk work).
- `100`: Full travel capacity.

*Verification*: When the battery charge reaches the threshold while plugged into AC power, `/sys/class/power_supply/BAT0/status` shifts from `Charging` to `Not charging` or `Full`.

### 2.3 Dual Fan RPM Telemetry
Dual fan sensors are exposed via the `asus-nb-wmi` hwmon interface:
```text
/sys/devices/platform/asus-nb-wmi/hwmon/hwmon[0-9]*/fan1_input  # CPU Fan (RPM)
/sys/devices/platform/asus-nb-wmi/hwmon/hwmon[0-9]*/fan2_input  # GPU Fan (RPM)
```

### 2.4 CPU Temperature (AMD k10temp)
AMD Ryzen Zen 3+ processors report package temperatures via the `k10temp` hwmon driver:
```text
/sys/class/hwmon/hwmon[0-9]*/temp1_input   # Tctl / Tdie in millidegrees Celsius
```

### 2.5 NVIDIA Discrete GPU Telemetry
Discrete GPU temperature, utilization, memory, and power metrics are acquired via:
- NVIDIA Management Library (`libnvidia-ml.so.1` / NVML)
- Dynamic Runtime D3cold check:
  ```text
  /sys/bus/pci/devices/0000:01:00.0/power/runtime_status
  ```
  If `runtime_status` is `suspended`, the card is asleep at 0W power draw. The application avoids invoking NVML to prevent unintended wakeups.

### 2.6 Keyboard Backlight & TUF Aura RGB
1. **Brightness**:
   ```text
   /sys/class/leds/asus::kbd_backlight/brightness      # 0 to 3
   /sys/class/leds/asus::kbd_backlight/max_brightness  # 3
   ```
2. **Aura Modes**:
   TUF keyboards accept single-zone Aura lighting commands through USB HID feature reports on `/dev/hidraw*` (vendor ID `0x0B05`, TUF keyboard product IDs) or through `/sys/class/leds/asus::kbd_backlight/kbd_rgb_mode`.
   Packet structure (16 bytes):
   - Byte 0: `0x5D` (TUF report ID)
   - Byte 1: `0xB4` (Aura command header)
   - Byte 2: Mode (`0x00` Static, `0x01` Breathing, `0x02` Strobing, `0x03` Color Cycle)
   - Byte 3: Red (`0x00` to `0xFF`)
   - Byte 4: Green (`0x00` to `0xFF`)
   - Byte 5: Blue (`0x00` to `0xFF`)
   - Byte 6: Speed (`0xE1` Slow, `0xEB` Normal, `0xF5` Fast)

---

## 3. Unsupported Features & Hardware Limitations on FA506NC

| Feature | Status | Reason & Behavior |
| :--- | :--- | :--- |
| **Advanced Optimus / Hardware MUX** | **Unsupported** | The FA506NC motherboard does not contain a multiplexer chip. Display output is physically routed through the AMD iGPU. The UI disables the "Ultimate / Dedicated MUX" switch and displays an explanatory tooltip. |
| **Fn-Lock Switch** | **Unsupported** | The embedded controller returns error `0xFFFF0000` (-65536) on the Fn-lock WMI query. The setting is hidden or disabled on this model. |
| **Per-Key RGB** | **Unsupported** | Hardware is single-zone RGB. The color picker controls the entire keyboard simultaneously. |
| **Mini-LED Multi-Zone** | **Unsupported** | Standard IPS edge-lit panel; mini-LED dimming is not applicable. |

---

## 4. Multi-Model Compatibility Matrix

| Laptop Family | Model Pattern | MUX Switch | Battery Limit | RGB Type | Fan Curve Control |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **TUF Gaming A15** | `FA506NC` | No (Hybrid only) | Yes (60/80/100) | Single Zone | Firmware profiles |
| **TUF Gaming A15** | `FA506QM/QR` | No (Hybrid only) | Yes (60/80/100) | Single Zone | Firmware profiles |
| **TUF Gaming A16** | `FA617NS/XS` | Yes (SmartAccess) | Yes (60/80/100) | Single / 4-Zone | Custom curve via asusctl |
| **ROG Zephyrus G14**| `GA402RJ/RK` | Yes (MUX) | Yes (60/80/100) | Per-Key / White | Custom curve via asusctl |
| **ROG Strix G15** | `G513Q/R` | Yes (MUX on newer) | Yes (60/80/100) | 4-Zone / Per-Key | Custom curve via asusctl |
