use crate::backend::HardwareBackend;
use crate::capabilities::HardwareCapabilities;
use crate::error::HardwareError;
use crate::models::{
    BatteryLimit, BatteryState, BatteryTelemetry, CpuTelemetry, FanTelemetry, GpuMode,
    GpuTelemetry, MemoryTelemetry, PerformanceMode, RgbConfig, SystemTelemetry,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MockHardwareBackend {
    capabilities: HardwareCapabilities,
    performance_mode: PerformanceMode,
    battery_limit: BatteryLimit,
    gpu_mode: GpuMode,
    rgb_config: RgbConfig,
    tick_count: u64,
    battery_pct: u8,
    ac_connected: bool,
    error_injection: Option<HardwareError>,
}

impl Default for MockHardwareBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MockHardwareBackend {
    pub fn new() -> Self {
        Self {
            capabilities: HardwareCapabilities::mock_fa506nc(),
            performance_mode: PerformanceMode::Balanced,
            battery_limit: BatteryLimit::Balanced,
            gpu_mode: GpuMode::Hybrid,
            rgb_config: RgbConfig::default(),
            tick_count: 0,
            battery_pct: 78,
            ac_connected: true,
            error_injection: None,
        }
    }

    pub fn set_ac_connected(&mut self, connected: bool) {
        self.ac_connected = connected;
    }

    pub fn inject_error(&mut self, err: Option<HardwareError>) {
        self.error_injection = err;
    }

    fn check_injected_error(&self) -> Result<(), HardwareError> {
        if let Some(err) = &self.error_injection {
            Err(err.clone())
        } else {
            Ok(())
        }
    }
}

impl HardwareBackend for MockHardwareBackend {
    fn detect_capabilities(&self) -> HardwareCapabilities {
        self.capabilities.clone()
    }

    fn read_telemetry(&mut self) -> Result<SystemTelemetry, HardwareError> {
        self.check_injected_error()?;
        self.tick_count += 1;

        let phase = (self.tick_count as f32) * 0.15;
        let sin_val = phase.sin();

        // CPU telemetry depending on mode
        let (base_temp, base_usage, max_fans) = match self.performance_mode {
            PerformanceMode::Silent => (42.0, 12.0, 2600),
            PerformanceMode::Balanced => (52.0, 24.0, 3400),
            PerformanceMode::Turbo => (68.0, 48.0, 4800),
            PerformanceMode::Custom => (55.0, 30.0, 3800),
        };

        let cpu_temp = base_temp + (sin_val * 6.0);
        let cpu_usage = (base_usage + (sin_val * 8.0)).clamp(2.0, 100.0);
        let cpu_freq = 3300.0 + (sin_val * 500.0);
        let cpu_fan_rpm = (max_fans as f32 * (cpu_usage / 100.0 + 0.4)).min(max_fans as f32) as u32;

        let gpu_active = match self.gpu_mode {
            GpuMode::Integrated => false,
            GpuMode::Hybrid => (self.tick_count % 8) < 4, // Toggles between active and asleep
            GpuMode::Dedicated => true,
        };

        let gpu = if gpu_active {
            let gpu_temp = 48.0 + (sin_val * 5.0);
            let gpu_usage = 15.0 + (sin_val * 10.0);
            GpuTelemetry {
                name: "NVIDIA GeForce RTX 3050 Laptop GPU".into(),
                is_active: true,
                temp_c: Some(gpu_temp),
                usage_pct: Some(gpu_usage),
                power_w: Some(25.0 + (sin_val * 8.0)),
                vram_used_mb: Some(820),
                vram_total_mb: Some(4096),
                fan_rpm: Some(cpu_fan_rpm.saturating_sub(200)),
            }
        } else {
            GpuTelemetry {
                name: "NVIDIA GeForce RTX 3050 Laptop GPU".into(),
                is_active: false,
                temp_c: None,
                usage_pct: Some(0.0),
                power_w: Some(0.0),
                vram_used_mb: None,
                vram_total_mb: Some(4096),
                fan_rpm: Some(0),
            }
        };

        let gpu_fan_rpm = gpu.fan_rpm.unwrap_or(0);

        // Battery state
        let bat_state = if self.ac_connected {
            if self.battery_pct >= self.battery_limit.as_pct() {
                BatteryState::NotCharging
            } else {
                BatteryState::Charging
            }
        } else {
            BatteryState::Discharging
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Ok(SystemTelemetry {
            cpu: CpuTelemetry {
                name: "AMD Ryzen 5 7535HS".into(),
                temp_c: cpu_temp,
                usage_pct: cpu_usage,
                freq_mhz: cpu_freq,
                power_w: Some(25.0 + (sin_val * 5.0)),
            },
            gpu,
            memory: MemoryTelemetry {
                total_mb: 16384,
                used_mb: 6144 + ((sin_val * 512.0) as u64),
                usage_pct: 37.5,
            },
            fans: FanTelemetry {
                cpu_fan_rpm,
                gpu_fan_rpm,
            },
            battery: BatteryTelemetry {
                percentage: self.battery_pct,
                state: bat_state,
                power_w: if self.ac_connected { 0.0 } else { 18.5 },
                health_pct: Some(99.0),
                cycle_count: Some(42),
                current_limit: self.battery_limit,
                ac_connected: self.ac_connected,
            },
            active_mode: self.performance_mode,
            active_gpu_mode: self.gpu_mode,
            timestamp_epoch_s: now,
        })
    }

    fn set_performance_mode(&mut self, mode: PerformanceMode) -> Result<(), HardwareError> {
        self.check_injected_error()?;
        self.performance_mode = mode;
        Ok(())
    }

    fn set_battery_limit(&mut self, limit: BatteryLimit) -> Result<(), HardwareError> {
        self.check_injected_error()?;
        self.battery_limit = limit;
        Ok(())
    }

    fn set_gpu_mode(&mut self, mode: GpuMode) -> Result<(), HardwareError> {
        self.check_injected_error()?;
        if mode == GpuMode::Dedicated && !self.capabilities.has_hardware_mux {
            return Err(HardwareError::Unsupported(
                "Dedicated MUX mode is unsupported on this laptop hardware (FA506NC)".into(),
            ));
        }
        self.gpu_mode = mode;
        Ok(())
    }

    fn set_keyboard_rgb(&mut self, config: &RgbConfig) -> Result<(), HardwareError> {
        self.check_injected_error()?;
        self.rgb_config = config.clone();
        Ok(())
    }

    fn set_keyboard_brightness(&mut self, brightness: u8) -> Result<(), HardwareError> {
        self.check_injected_error()?;
        if brightness > 3 {
            return Err(HardwareError::InvalidValue("Brightness must be 0-3".into()));
        }
        self.rgb_config.brightness = brightness;
        Ok(())
    }

    fn get_performance_mode(&self) -> PerformanceMode {
        self.performance_mode
    }

    fn get_battery_limit(&self) -> BatteryLimit {
        self.battery_limit
    }

    fn get_gpu_mode(&self) -> GpuMode {
        self.gpu_mode
    }

    fn get_keyboard_rgb(&self) -> RgbConfig {
        self.rgb_config.clone()
    }

    fn is_simulation(&self) -> bool {
        true
    }
}
