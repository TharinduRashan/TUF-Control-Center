use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::models::GpuMode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    pub model_name: String,
    pub sys_vendor: String,
    pub has_throttle_thermal_policy: bool,
    pub has_platform_profile: bool,
    pub has_battery_threshold: bool,
    pub supported_battery_limits: Vec<u8>,
    pub has_cpu_epp: bool,
    pub has_dual_fans: bool,
    pub has_custom_fan_curves: bool,
    pub has_keyboard_rgb: bool,
    pub has_hardware_mux: bool,
    pub supported_gpu_modes: Vec<GpuMode>,
    pub supported_refresh_rates: Vec<u32>,
    pub is_simulation: bool,
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            model_name: "Generic Linux Laptop".into(),
            sys_vendor: "Unknown".into(),
            has_throttle_thermal_policy: false,
            has_platform_profile: false,
            has_battery_threshold: false,
            supported_battery_limits: vec![100],
            has_cpu_epp: false,
            has_dual_fans: false,
            has_custom_fan_curves: false,
            has_keyboard_rgb: false,
            has_hardware_mux: false,
            supported_gpu_modes: vec![GpuMode::Hybrid],
            supported_refresh_rates: vec![60],
            is_simulation: false,
        }
    }
}

impl HardwareCapabilities {
    /// Returns simulated capabilities matching the ASUS TUF Gaming A15 FA506NC
    pub fn mock_fa506nc() -> Self {
        Self {
            model_name: "ASUS TUF Gaming A15 FA506NC_FA506NC".into(),
            sys_vendor: "ASUSTeK COMPUTER INC.".into(),
            has_throttle_thermal_policy: true,
            has_platform_profile: true,
            has_battery_threshold: true,
            supported_battery_limits: vec![60, 80, 100],
            has_cpu_epp: true,
            has_dual_fans: true,
            has_custom_fan_curves: false, // TUF models use firmware policies
            has_keyboard_rgb: true,
            has_hardware_mux: false, // FA506NC has no physical MUX switch
            supported_gpu_modes: vec![GpuMode::Hybrid, GpuMode::Integrated],
            supported_refresh_rates: vec![60, 144],
            is_simulation: true,
        }
    }

    /// Probes Linux kernel sysfs and DMI for actual hardware support
    pub fn probe() -> Self {
        let model_name = read_dmi_string("/sys/class/dmi/id/product_name")
            .unwrap_or_else(|| "Unknown Hardware".into());
        let sys_vendor = read_dmi_string("/sys/class/dmi/id/sys_vendor")
            .unwrap_or_else(|| "Unknown Vendor".into());

        let is_asus = sys_vendor.to_lowercase().contains("asus");

        let has_throttle_thermal_policy = Path::new("/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy").exists();
        let has_platform_profile = Path::new("/sys/firmware/acpi/platform_profile").exists();

        let has_battery_threshold = Path::new("/sys/class/power_supply/BAT0/charge_control_end_threshold").exists()
            || Path::new("/sys/class/power_supply/BAT1/charge_control_end_threshold").exists();

        let has_cpu_epp = Path::new("/sys/devices/system/cpu/cpufreq/policy0/energy_performance_preference").exists();

        // Check for dual fans in asus-nb-wmi hwmon
        let has_dual_fans = probe_dual_fans();

        // Check for keyboard RGB
        let has_keyboard_rgb = Path::new("/sys/class/leds/asus::kbd_backlight/brightness").exists()
            || Path::new("/sys/class/leds/asus::kbd_backlight/kbd_rgb_mode").exists();

        // Hardware MUX check
        let has_hardware_mux = Path::new("/sys/devices/platform/asus-nb-wmi/gpu_mux").exists()
            || Path::new("/sys/bus/platform/devices/asus-nb-wmi/dgpu_disable").exists();

        let mut gpu_modes = vec![GpuMode::Hybrid];
        // If supergfxctl or dgpu power cut is present, Integrated is supported
        if Path::new("/run/dbus/system_bus_socket").exists() {
            gpu_modes.push(GpuMode::Integrated);
        }
        if has_hardware_mux {
            gpu_modes.push(GpuMode::Dedicated);
        }

        let supported_battery_limits = if has_battery_threshold {
            vec![60, 80, 100]
        } else {
            vec![100]
        };

        let mut refresh_rates = vec![60];
        if model_name.contains("FA506") || model_name.contains("FA617") || model_name.contains("FX506") {
            refresh_rates.push(144);
        }

        Self {
            model_name,
            sys_vendor,
            has_throttle_thermal_policy: has_throttle_thermal_policy || (is_asus && has_platform_profile),
            has_platform_profile,
            has_battery_threshold,
            supported_battery_limits,
            has_cpu_epp,
            has_dual_fans,
            has_custom_fan_curves: false,
            has_keyboard_rgb,
            has_hardware_mux,
            supported_gpu_modes: gpu_modes,
            supported_refresh_rates: refresh_rates,
            is_simulation: false,
        }
    }
}

fn read_dmi_string(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn probe_dual_fans() -> bool {
    let hwmon_base = Path::new("/sys/devices/platform/asus-nb-wmi/hwmon");
    if let Ok(entries) = fs::read_dir(hwmon_base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.join("fan1_input").exists() && path.join("fan2_input").exists() {
                return true;
            }
        }
    }
    false
}
