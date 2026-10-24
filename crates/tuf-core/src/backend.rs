use crate::capabilities::HardwareCapabilities;
use crate::error::HardwareError;
use crate::models::{BatteryLimit, GpuMode, PerformanceMode, RgbConfig, SystemTelemetry};

pub trait HardwareBackend: Send + Sync {
    fn detect_capabilities(&self) -> HardwareCapabilities;
    fn read_telemetry(&mut self) -> Result<SystemTelemetry, HardwareError>;
    fn set_performance_mode(&mut self, mode: PerformanceMode) -> Result<(), HardwareError>;
    fn set_battery_limit(&mut self, limit: BatteryLimit) -> Result<(), HardwareError>;
    fn set_gpu_mode(&mut self, mode: GpuMode) -> Result<(), HardwareError>;
    fn set_keyboard_rgb(&mut self, config: &RgbConfig) -> Result<(), HardwareError>;
    fn set_keyboard_brightness(&mut self, brightness: u8) -> Result<(), HardwareError>;
    fn get_performance_mode(&self) -> PerformanceMode;
    fn get_battery_limit(&self) -> BatteryLimit;
    fn get_gpu_mode(&self) -> GpuMode;
    fn get_keyboard_rgb(&self) -> RgbConfig;
    fn is_simulation(&self) -> bool;
}
