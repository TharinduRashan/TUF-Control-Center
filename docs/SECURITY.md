# Security Architecture & Policies

`TUF Control Center` implements a strict least-privilege security model designed to eliminate common vulnerabilities found in hardware utilities (such as running entire graphical environments as root, executing unsanitized shell commands, or exposing arbitrary file writes).

---

## 1. Privilege Separation

```
[ Unprivileged Desktop Session ]
       tuf-gui / tuf-tray / tuf-cli
                 │
                 │ D-Bus Method Call (Unix Domain Socket)
                 ▼
[ Privileged System Service ]
            tuf-daemon
                 │
                 ├── PolicyKit (polkit-1) Authorization Check
                 ├── Strict Input Validation & Allowlist Matching
                 └── Atomic Sysfs Write / Kernel Interface
```

### Key Principles:
1. **Never Run the GUI as Root**:
   `tuf-gui` runs exclusively as the standard unprivileged desktop user. It has no elevated capabilities and communicates with the system daemon over the standard system D-Bus bus (`unix:path=/run/dbus/system_bus_socket`).
2. **Narrowly Scoped Daemon**:
   `tuf-daemon` only exposes specific hardware control methods:
   - `SetThrottleThermalPolicy(mode: u32)`
   - `SetPlatformProfile(profile: String)`
   - `SetBatteryChargeLimit(limit_pct: u32)`
   - `SetCpuEnergyPerformancePreference(epp: String)`
   - `SetKeyboardBrightness(brightness: u32)`
   - `SetKeyboardRgb(mode: u32, r: u8, g: u8, b: u8, speed: u32)`
   - `SetGpuMode(mode: String)`

---

## 2. Input Validation & Whitelisting

The privileged daemon strictly rejects any input outside predefined bounds:

| Parameter | Allowed Values / Validation Rules |
| :--- | :--- |
| `throttle_thermal_policy` | Integers `0` (Balanced), `1` (Turbo), `2` (Silent). Any other value is rejected with `InvalidParameter`. |
| `platform_profile` | String matching `quiet`, `balanced`, or `performance`. |
| `charge_control_end_threshold` | Integers `60`, `80`, or `100`. Values like `0`, `50`, `99`, or negative numbers are rejected. |
| `energy_performance_preference` | String matching `power`, `balance_power`, `balance_performance`, or `performance`. |
| `keyboard_brightness` | Integers `0` through `3`. |
| `keyboard_rgb` | Mode `0`..`3`, RGB components `0`..`255`, speed `0`..`2`. |
| `gpu_mode` | String matching `hybrid`, `integrated`, or `dedicated`. |

### Path Traversal & Symlink Protection:
All file operations performed by `tuf-daemon` target strictly defined canonical paths.
Files are opened with `O_NOFOLLOW` or validated against canonical base paths under `/sys/devices/platform/asus-nb-wmi/` and `/sys/class/power_supply/`.
User-supplied file paths are never accepted.

---

## 3. PolicyKit (Polkit) Configuration

The PolicyKit policy file is installed to `/usr/share/polkit-1/actions/org.tuf.controlcenter.policy`.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE policyconfig PUBLIC
 "-//freedesktop//DTD PolicyKit Policy Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/PolicyKit/1.0/policyconfig.dtd">
<policyconfig>
  <vendor>TUF Control Center</vendor>
  <vendor_url>https://github.com/tuf-control-center/tuf-control-center</vendor_url>

  <action id="org.tuf.controlcenter.set-hardware-settings">
    <description>Modify ASUS hardware settings (performance, battery, fans)</description>
    <message>Authentication is required to adjust hardware power and thermal profiles</message>
    <defaults>
      <allow_any>auth_admin</allow_any>
      <allow_inactive>auth_admin</allow_inactive>
      <allow_active>yes</allow_active>
    </defaults>
  </action>
</policyconfig>
```

- **Active local console session**: `<allow_active>yes</allow_active>` allows the logged-in desktop user to adjust performance profiles and battery limits seamlessly without repetitive password prompts.
- **Remote / SSH / Inactive sessions**: Require administrative authentication (`auth_admin`) to prevent remote tampering.

---

## 4. No Shell Execution

`tuf-daemon` and `tuf-gui` never execute `/bin/sh -c` or invoke shell strings with unsanitized arguments.
External tool invocations (such as calling `supergfxctl` or querying `pkcheck`) use direct process spawning (`std::process::Command`) with explicit argument vectors.

---

## 5. Audit Logging

Every state-changing action is logged to the systemd journal with structured metadata:
- Originating client UID / PID
- Target parameter name and requested value
- Previous parameter value
- Success or failure status
