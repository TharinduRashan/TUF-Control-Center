use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;
use tuf_core::error::HardwareError;

pub struct SysfsController {
    thermal_policy_path: PathBuf,
    platform_profile_path: PathBuf,
    battery_limit_path: Option<PathBuf>,
    kbd_brightness_path: Option<PathBuf>,
    simulation: bool,
}

impl SysfsController {
    pub fn new(simulation: bool) -> Self {
        let thermal_policy_path =
            PathBuf::from("/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy");
        let platform_profile_path = PathBuf::from("/sys/firmware/acpi/platform_profile");

        let battery_limit_path = [
            "/sys/class/power_supply/BAT0/charge_control_end_threshold",
            "/sys/class/power_supply/BAT1/charge_control_end_threshold",
            "/sys/class/power_supply/BATC/charge_control_end_threshold",
        ]
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists());

        let kbd_brightness_path = [
            "/sys/class/leds/asus::kbd_backlight/brightness",
            "/sys/class/leds/asus-nb-wmi::kbd_backlight/brightness",
        ]
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists());

        Self {
            thermal_policy_path,
            platform_profile_path,
            battery_limit_path,
            kbd_brightness_path,
            simulation,
        }
    }

    pub fn set_throttle_thermal_policy(&self, policy: u32) -> Result<(), HardwareError> {
        if policy > 2 {
            return Err(HardwareError::InvalidValue(format!(
                "Invalid thermal policy code: {policy}. Must be 0 (Balanced), 1 (Turbo), or 2 (Silent)"
            )));
        }

        if self.simulation {
            info!("[SYSFS SIM] Written thermal policy: {}", policy);
            return Ok(());
        }

        if !self.thermal_policy_path.exists() {
            return Err(HardwareError::Unsupported(format!(
                "throttle_thermal_policy sysfs interface not found at {}. Ensure asus-nb-wmi kernel module is loaded.",
                self.thermal_policy_path.display()
            )));
        }

        fs::write(&self.thermal_policy_path, policy.to_string()).map_err(|e| {
            HardwareError::PermissionDenied(format!("Failed to write thermal policy: {e}"))
        })?;

        info!("Applied throttle_thermal_policy = {}", policy);
        Ok(())
    }

    pub fn get_throttle_thermal_policy(&self) -> Result<u32, HardwareError> {
        if self.simulation {
            return Ok(0); // Balanced default in simulation
        }

        if !self.thermal_policy_path.exists() {
            return Err(HardwareError::Unsupported(format!(
                "throttle_thermal_policy sysfs interface not found at {}",
                self.thermal_policy_path.display()
            )));
        }

        let raw = fs::read_to_string(&self.thermal_policy_path)
            .map_err(|e| HardwareError::Io(format!("Failed to read thermal policy: {e}")))?;
        raw.trim()
            .parse::<u32>()
            .map_err(|e| HardwareError::Parse(format!("Failed to parse thermal policy: {e}")))
    }

    pub fn set_platform_profile(&self, profile: &str) -> Result<(), HardwareError> {
        let valid_profiles = ["quiet", "balanced", "performance"];
        if !valid_profiles.contains(&profile) {
            return Err(HardwareError::InvalidValue(format!(
                "Invalid platform profile: '{profile}'. Must be quiet, balanced, or performance."
            )));
        }

        if self.simulation {
            info!("[SYSFS SIM] Written platform profile: {}", profile);
            return Ok(());
        }

        if !self.platform_profile_path.exists() {
            return Err(HardwareError::Unsupported(format!(
                "ACPI platform_profile interface not found at {}",
                self.platform_profile_path.display()
            )));
        }

        fs::write(&self.platform_profile_path, profile).map_err(|e| {
            HardwareError::PermissionDenied(format!("Failed to write platform profile: {e}"))
        })?;

        info!("Applied platform_profile = {}", profile);
        Ok(())
    }

    pub fn get_platform_profile(&self) -> Result<String, HardwareError> {
        if self.simulation {
            return Ok("balanced".into());
        }

        if !self.platform_profile_path.exists() {
            return Err(HardwareError::Unsupported(format!(
                "ACPI platform_profile interface not found at {}",
                self.platform_profile_path.display()
            )));
        }

        let raw = fs::read_to_string(&self.platform_profile_path)
            .map_err(|e| HardwareError::Io(format!("Failed to read platform profile: {e}")))?;
        Ok(raw.trim().to_string())
    }

    pub fn set_battery_limit(&self, limit_pct: u32) -> Result<(), HardwareError> {
        if limit_pct != 60 && limit_pct != 80 && limit_pct != 100 {
            return Err(HardwareError::InvalidValue(format!(
                "Invalid charge limit: {limit_pct}%. Only 60, 80, or 100 are permitted."
            )));
        }

        if self.simulation {
            info!("[SYSFS SIM] Written battery limit: {}%", limit_pct);
            return Ok(());
        }

        if let Some(path) = &self.battery_limit_path {
            fs::write(path, limit_pct.to_string()).map_err(|e| {
                HardwareError::PermissionDenied(format!("Failed to write charge limit: {e}"))
            })?;

            info!(
                "Applied battery charge_control_end_threshold = {}%",
                limit_pct
            );
            Ok(())
        } else {
            Err(HardwareError::Unsupported("Battery charge limit is unsupported on this hardware (no charge_control_end_threshold node)".into()))
        }
    }

    pub fn get_battery_limit(&self) -> Result<u32, HardwareError> {
        if self.simulation {
            return Ok(80);
        }

        if let Some(path) = &self.battery_limit_path {
            let raw = fs::read_to_string(path)
                .map_err(|e| HardwareError::Io(format!("Failed to read battery limit: {e}")))?;
            raw.trim()
                .parse::<u32>()
                .map_err(|e| HardwareError::Parse(format!("Failed to parse battery limit: {e}")))
        } else {
            Err(HardwareError::Unsupported(
                "Battery charge limit interface not found".into(),
            ))
        }
    }

    pub fn set_cpu_epp(&self, epp: &str) -> Result<(), HardwareError> {
        let valid = [
            "power",
            "balance_power",
            "balance_performance",
            "performance",
        ];
        if !valid.contains(&epp) {
            return Err(HardwareError::InvalidValue(format!(
                "Invalid CPU EPP value: {epp}"
            )));
        }

        if self.simulation {
            info!("[SYSFS SIM] Written CPU EPP: {}", epp);
            return Ok(());
        }

        let policy_dir = Path::new("/sys/devices/system/cpu/cpufreq");
        if !policy_dir.exists() {
            return Err(HardwareError::Unsupported(
                "cpufreq subsystem not available".into(),
            ));
        }

        let entries = match fs::read_dir(policy_dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(HardwareError::Io(format!(
                    "Failed to read cpufreq directory: {e}"
                )))
            }
        };

        let mut applied_count = 0;
        for entry in entries.flatten() {
            let p = entry.path().join("energy_performance_preference");
            if p.exists() && fs::write(&p, epp).is_ok() {
                applied_count += 1;
            }
        }

        if applied_count == 0 {
            return Err(HardwareError::Unsupported(
                "No CPU scaling policies support energy_performance_preference".into(),
            ));
        }

        info!(
            "Applied CPU energy_performance_preference = {} to {} policies",
            epp, applied_count
        );
        Ok(())
    }

    pub fn set_keyboard_brightness(&self, level: u32) -> Result<(), HardwareError> {
        if level > 3 {
            return Err(HardwareError::InvalidValue(format!(
                "Invalid keyboard brightness: {level}. Must be 0 to 3."
            )));
        }

        if self.simulation {
            info!("[SYSFS SIM] Written keyboard brightness: {}", level);
            return Ok(());
        }

        if let Some(path) = &self.kbd_brightness_path {
            fs::write(path, level.to_string()).map_err(|e| {
                HardwareError::PermissionDenied(format!("Failed to write kbd brightness: {e}"))
            })?;
            info!("Applied keyboard backlight brightness = {}", level);
            Ok(())
        } else {
            Err(HardwareError::Unsupported(
                "Keyboard backlight sysfs path not found (ensure asus-nb-wmi is loaded)".into(),
            ))
        }
    }
}
