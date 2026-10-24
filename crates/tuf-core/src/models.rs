use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use crate::error::HardwareError;

/// Operating and performance profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceMode {
    Silent,
    Balanced,
    Turbo,
    Custom,
}

impl PerformanceMode {
    /// Maps to ASUS throttle_thermal_policy sysfs value
    /// 0 = Standard/Balanced, 1 = Overboost/Turbo, 2 = Silent
    pub fn to_throttle_thermal_policy(self) -> u32 {
        match self {
            Self::Balanced | Self::Custom => 0,
            Self::Turbo => 1,
            Self::Silent => 2,
        }
    }

    /// Maps from ASUS throttle_thermal_policy
    pub fn from_throttle_thermal_policy(val: u32) -> Result<Self, HardwareError> {
        match val {
            0 => Ok(Self::Balanced),
            1 => Ok(Self::Turbo),
            2 => Ok(Self::Silent),
            other => Err(HardwareError::InvalidValue(format!(
                "Unknown throttle thermal policy code: {other}"
            ))),
        }
    }

    /// Maps to Linux ACPI platform_profile string
    pub fn to_platform_profile(self) -> &'static str {
        match self {
            Self::Silent => "quiet",
            Self::Balanced | Self::Custom => "balanced",
            Self::Turbo => "performance",
        }
    }

    /// Maps to AMD Ryzen energy_performance_preference (EPP)
    pub fn to_epp(self) -> &'static str {
        match self {
            Self::Silent => "power",
            Self::Balanced => "balance_performance",
            Self::Turbo | Self::Custom => "performance",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Silent => "Silent",
            Self::Balanced => "Balanced",
            Self::Turbo => "Turbo",
            Self::Custom => "Custom",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Silent => "Quiet fan curves, lower power limits, ideal for battery & quiet environments",
            Self::Balanced => "Optimal balance between acoustics, thermals, and everyday performance",
            Self::Turbo => "Maximum fan speeds and boosted CPU/GPU power ceilings for heavy gaming",
            Self::Custom => "Custom user-defined fan curves and power targets where supported",
        }
    }
}

impl fmt::Display for PerformanceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for PerformanceMode {
    type Err = HardwareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "silent" | "quiet" | "2" => Ok(Self::Silent),
            "balanced" | "normal" | "0" => Ok(Self::Balanced),
            "turbo" | "performance" | "1" => Ok(Self::Turbo),
            "custom" => Ok(Self::Custom),
            other => Err(HardwareError::InvalidValue(format!("Invalid performance mode: {other}"))),
        }
    }
}

/// Supported hardware battery charge thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryLimit {
    #[serde(rename = "60")]
    MaximumLifespan = 60,
    #[serde(rename = "80")]
    Balanced = 80,
    #[serde(rename = "100")]
    Full = 100,
}

impl BatteryLimit {
    pub fn as_pct(self) -> u8 {
        self as u8
    }

    pub fn from_pct(pct: u8) -> Result<Self, HardwareError> {
        match pct {
            60 => Ok(Self::MaximumLifespan),
            80 => Ok(Self::Balanced),
            100 => Ok(Self::Full),
            other => Err(HardwareError::InvalidValue(format!(
                "Invalid charge limit: {other}%. Must be 60, 80, or 100."
            ))),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::MaximumLifespan => "60% - Maximum battery lifespan for predominantly plugged-in desktop use",
            Self::Balanced => "80% - Recommended for balanced daily use and reduced battery wear",
            Self::Full => "100% - Full capacity for maximum portable runtime away from an outlet",
        }
    }
}

impl fmt::Display for BatteryLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.as_pct())
    }
}

/// Graphics configurations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuMode {
    /// Hybrid graphics: AMD iGPU drives display, NVIDIA dGPU on demand via PRIME offload
    Hybrid,
    /// Integrated graphics only: dGPU powered down (runtime D3cold or PCI power cut)
    Integrated,
    /// Dedicated GPU mode (requires hardware MUX switch or dedicated routing)
    Dedicated,
}

impl GpuMode {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Hybrid => "Hybrid (Optimus / PRIME)",
            Self::Integrated => "Integrated (Eco)",
            Self::Dedicated => "Dedicated (Ultimate MUX)",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Hybrid => "AMD iGPU runs desktop; NVIDIA dGPU sleeps when idle and powers on for gaming",
            Self::Integrated => "NVIDIA GPU completely powered off for maximum battery life",
            Self::Dedicated => "Direct dGPU connection (requires physical hardware MUX switch)",
        }
    }
}

impl fmt::Display for GpuMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for GpuMode {
    type Err = HardwareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hybrid" | "optimus" | "prime" => Ok(Self::Hybrid),
            "integrated" | "eco" | "igpu" => Ok(Self::Integrated),
            "dedicated" | "ultimate" | "dgpu" => Ok(Self::Dedicated),
            other => Err(HardwareError::InvalidValue(format!("Invalid GPU mode: {other}"))),
        }
    }
}

/// RGB Backlight Animation Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RgbMode {
    Static = 0,
    Breathing = 1,
    Strobing = 2,
    ColorCycle = 3,
}

impl RgbMode {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::Breathing => "Breathing",
            Self::Strobing => "Strobing",
            Self::ColorCycle => "Color Cycle",
        }
    }
}

/// RGB Animation Speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RgbSpeed {
    Slow = 0,
    Normal = 1,
    Fast = 2,
}

impl RgbSpeed {
    pub fn to_hid_byte(self) -> u8 {
        match self {
            Self::Slow => 0xE1,
            Self::Normal => 0xEB,
            Self::Fast => 0xF5,
        }
    }
}

/// Color representation in RGB space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 128, b: 255 };
    pub const TUF_ORANGE: Self = Self { r: 255, g: 128, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_hex(hex: &str) -> Result<Self, HardwareError> {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() != 6 {
            return Err(HardwareError::InvalidValue(format!("Invalid hex color: {hex}")));
        }
        let r = u8::from_str_radix(&clean[0..2], 16)
            .map_err(|e| HardwareError::InvalidValue(e.to_string()))?;
        let g = u8::from_str_radix(&clean[2..4], 16)
            .map_err(|e| HardwareError::InvalidValue(e.to_string()))?;
        let b = u8::from_str_radix(&clean[4..6], 16)
            .map_err(|e| HardwareError::InvalidValue(e.to_string()))?;
        Ok(Self { r, g, b })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbConfig {
    pub mode: RgbMode,
    pub color: RgbColor,
    pub speed: RgbSpeed,
    pub brightness: u8,
}

impl Default for RgbConfig {
    fn default() -> Self {
        Self {
            mode: RgbMode::Static,
            color: RgbColor::TUF_ORANGE,
            speed: RgbSpeed::Normal,
            brightness: 3,
        }
    }
}

/// Power supply charging state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryState {
    Charging,
    Discharging,
    NotCharging,
    Full,
    Unknown,
}

impl fmt::Display for BatteryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Charging => write!(f, "Charging"),
            Self::Discharging => write!(f, "Discharging"),
            Self::NotCharging => write!(f, "Not Charging (Limit Active)"),
            Self::Full => write!(f, "Full"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Point on a custom fan curve
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanCurvePoint {
    pub temp_c: u8,
    pub speed_pct: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanCurve {
    pub points: Vec<FanCurvePoint>,
}

impl FanCurve {
    /// Validates fan curve safety requirements:
    /// - At least 4 and at most 8 points
    /// - Temperatures must be monotonically strictly increasing
    /// - Temperatures between 30°C and 105°C
    /// - Speeds between 0% and 100%
    /// - Thermal safety: Speed cannot be 0% at or above 70°C
    pub fn validate(&self) -> Result<(), HardwareError> {
        if self.points.len() < 4 || self.points.len() > 8 {
            return Err(HardwareError::Validation(
                "Fan curve must contain between 4 and 8 curve points".into(),
            ));
        }

        let mut prev_temp = 0u8;
        for (idx, pt) in self.points.iter().enumerate() {
            if pt.temp_c < 30 || pt.temp_c > 105 {
                return Err(HardwareError::Validation(format!(
                    "Temperature {}°C at point {} is out of safe range (30-105°C)",
                    pt.temp_c, idx
                )));
            }
            if pt.speed_pct > 100 {
                return Err(HardwareError::Validation(format!(
                    "Speed {}% at point {} exceeds 100%",
                    pt.speed_pct, idx
                )));
            }
            if pt.temp_c <= prev_temp {
                return Err(HardwareError::Validation(format!(
                    "Temperature must strictly increase. Point {} ({}°C) <= previous ({}°C)",
                    idx, pt.temp_c, prev_temp
                )));
            }
            if pt.temp_c >= 70 && pt.speed_pct == 0 {
                return Err(HardwareError::SafetyProtection(format!(
                    "Safety violation: Fans cannot be set to 0% at {}°C",
                    pt.temp_c
                )));
            }
            prev_temp = pt.temp_c;
        }

        Ok(())
    }
}

/// System telemetry data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CpuTelemetry {
    pub name: String,
    pub temp_c: f32,
    pub usage_pct: f32,
    pub freq_mhz: f32,
    pub power_w: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpuTelemetry {
    pub name: String,
    pub is_active: bool,
    pub temp_c: Option<f32>,
    pub usage_pct: Option<f32>,
    pub power_w: Option<f32>,
    pub vram_used_mb: Option<u64>,
    pub vram_total_mb: Option<u64>,
    pub fan_rpm: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryTelemetry {
    pub total_mb: u64,
    pub used_mb: u64,
    pub usage_pct: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FanTelemetry {
    pub cpu_fan_rpm: u32,
    pub gpu_fan_rpm: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryTelemetry {
    pub percentage: u8,
    pub state: BatteryState,
    pub power_w: f32,
    pub health_pct: Option<f32>,
    pub cycle_count: Option<u32>,
    pub current_limit: BatteryLimit,
    pub ac_connected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemTelemetry {
    pub cpu: CpuTelemetry,
    pub gpu: GpuTelemetry,
    pub memory: MemoryTelemetry,
    pub fans: FanTelemetry,
    pub battery: BatteryTelemetry,
    pub active_mode: PerformanceMode,
    pub active_gpu_mode: GpuMode,
    pub timestamp_epoch_s: u64,
}
