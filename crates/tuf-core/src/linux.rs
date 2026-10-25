use std::sync::Mutex;
use tracing::{info, warn};
use zbus::blocking::Connection;

use crate::backend::HardwareBackend;
use crate::capabilities::HardwareCapabilities;
use crate::error::HardwareError;
use crate::models::{BatteryLimit, GpuMode, PerformanceMode, RgbConfig, SystemTelemetry};
use crate::telemetry::LinuxTelemetryReader;

const DESTINATION: &str = "org.tuf.ControlCenter";
const PATH: &str = "/org/tuf/ControlCenter";
const INTERFACE: &str = "org.tuf.ControlCenter";

pub struct LinuxHardwareBackend {
    telemetry_reader: LinuxTelemetryReader,
    capabilities: HardwareCapabilities,
    cached_mode: PerformanceMode,
    cached_battery_limit: BatteryLimit,
    cached_gpu_mode: GpuMode,
    cached_rgb: RgbConfig,
    _cached_kbd_brightness: u8,
    connection: Mutex<Option<Connection>>,
}

impl Default for LinuxHardwareBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxHardwareBackend {
    pub fn new() -> Self {
        let telemetry_reader = LinuxTelemetryReader::new();
        let capabilities = HardwareCapabilities::probe();

        let conn = Connection::system().or_else(|_| Connection::session()).ok();
        if conn.is_none() {
            warn!("Could not establish initial D-Bus connection to tuf-daemon. Commands will attempt reconnection on demand.");
        }

        Self {
            telemetry_reader,
            capabilities,
            cached_mode: PerformanceMode::Balanced,
            cached_battery_limit: BatteryLimit::Balanced,
            cached_gpu_mode: GpuMode::Hybrid,
            cached_rgb: RgbConfig::default(),
            _cached_kbd_brightness: 3,
            connection: Mutex::new(conn),
        }
    }

    fn call_daemon<B: serde::Serialize + zbus::zvariant::Type>(
        &self,
        method: &str,
        body: &B,
    ) -> Result<(), HardwareError> {
        let mut lock = self
            .connection
            .lock()
            .map_err(|_| HardwareError::DBus("Connection lock poisoned".into()))?;

        if lock.is_none() {
            *lock = Connection::system().or_else(|_| Connection::session()).ok();
        }

        let conn = lock.as_ref().ok_or_else(|| {
            HardwareError::DBus(
                "TUF Daemon is not running. Please start tuf-daemon.service.".into(),
            )
        })?;

        conn.call_method(Some(DESTINATION), PATH, Some(INTERFACE), method, body)
            .map_err(|e| HardwareError::DBus(format!("D-Bus call to {method} failed: {e}")))?;

        Ok(())
    }
}

impl HardwareBackend for LinuxHardwareBackend {
    fn detect_capabilities(&self) -> HardwareCapabilities {
        self.capabilities.clone()
    }

    fn read_telemetry(&mut self) -> Result<SystemTelemetry, HardwareError> {
        self.telemetry_reader
            .read_system_telemetry(self.cached_mode, self.cached_gpu_mode)
    }

    fn set_performance_mode(&mut self, mode: PerformanceMode) -> Result<(), HardwareError> {
        info!("Applying hardware performance mode: {}", mode);
        let policy = mode.to_throttle_thermal_policy();

        // throttle_thermal_policy is the primary ASUS hardware control
        if let Err(e) = self.call_daemon("set_throttle_thermal_policy", &(policy,)) {
            warn!("asus-nb-wmi throttle_thermal_policy failed ({e}), falling back to ACPI platform_profile");
            let profile = mode.to_platform_profile().to_string();
            self.call_daemon("set_platform_profile", &(profile,))?;
        }

        let epp = mode.to_epp().to_string();
        let _ = self.call_daemon("set_cpu_epp", &(epp,));

        self.cached_mode = mode;
        Ok(())
    }

    fn set_battery_limit(&mut self, limit: BatteryLimit) -> Result<(), HardwareError> {
        info!(
            "Applying hardware battery charge threshold: {}%",
            limit.as_pct()
        );
        let limit_val = limit.as_pct() as u32;
        self.call_daemon("set_battery_limit", &(limit_val,))?;
        self.cached_battery_limit = limit;
        Ok(())
    }

    fn set_gpu_mode(&mut self, mode: GpuMode) -> Result<(), HardwareError> {
        info!("Applying GPU operating mode: {}", mode);
        let mode_str = match mode {
            GpuMode::Hybrid => "hybrid",
            GpuMode::Integrated => "integrated",
            GpuMode::Dedicated => "dedicated",
        }
        .to_string();
        self.call_daemon("set_gpu_mode", &(mode_str,))?;
        self.cached_gpu_mode = mode;
        Ok(())
    }

    fn set_keyboard_rgb(&mut self, config: &RgbConfig) -> Result<(), HardwareError> {
        info!("Applying keyboard RGB lighting: {:?}", config);
        let mode = config.mode as u32;
        let r = config.color.r;
        let g = config.color.g;
        let b = config.color.b;
        let speed = config.speed as u32;
        self.call_daemon("set_keyboard_rgb", &(mode, r, g, b, speed))?;
        self.cached_rgb = config.clone();
        Ok(())
    }

    fn set_keyboard_brightness(&mut self, brightness: u8) -> Result<(), HardwareError> {
        info!("Applying keyboard backlight brightness: {}", brightness);
        let val = brightness as u32;
        self.call_daemon("set_keyboard_brightness", &(val,))?;
        self._cached_kbd_brightness = brightness;
        Ok(())
    }

    fn get_performance_mode(&self) -> PerformanceMode {
        self.cached_mode
    }

    fn get_battery_limit(&self) -> BatteryLimit {
        self.cached_battery_limit
    }

    fn get_gpu_mode(&self) -> GpuMode {
        self.cached_gpu_mode
    }

    fn get_keyboard_rgb(&self) -> RgbConfig {
        self.cached_rgb.clone()
    }

    fn is_simulation(&self) -> bool {
        false
    }
}
