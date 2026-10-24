use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use crate::error::HardwareError;
use crate::models::{BatteryLimit, GpuMode, PerformanceMode, RgbConfig};

const CURRENT_CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u32,
    pub performance_mode: PerformanceMode,
    pub battery_limit: BatteryLimit,
    pub gpu_mode: GpuMode,
    pub keyboard_rgb: RgbConfig,
    pub refresh_rate: u32,
    pub start_minimized: bool,
    pub tray_icon: bool,
    pub polling_interval_ms: u64,
    pub simulation_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION,
            performance_mode: PerformanceMode::Balanced,
            battery_limit: BatteryLimit::Balanced, // 80% default for optimal longevity
            gpu_mode: GpuMode::Hybrid,
            keyboard_rgb: RgbConfig::default(),
            refresh_rate: 144,
            start_minimized: false,
            tray_icon: true,
            polling_interval_ms: 1000,
            simulation_mode: false,
        }
    }
}

impl AppConfig {
    pub fn default_path() -> PathBuf {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let home = std::env::var_os("HOME").unwrap_or_default();
                PathBuf::from(home).join(".config")
            });
        base.join("tuf-control-center").join("config.toml")
    }

    pub fn load_or_default() -> Self {
        let path = Self::default_path();
        if path.exists() {
            match Self::load_from_path(&path) {
                Ok(cfg) => cfg,
                Err(e) => {
                    warn!("Failed to load config from {}: {e}. Creating backup and loading defaults.", path.display());
                    let bak_path = path.with_extension("toml.bak");
                    let _ = fs::copy(&path, &bak_path);
                    let def = Self::default();
                    let _ = def.save_to_path(&path);
                    def
                }
            }
        } else {
            let def = Self::default();
            let _ = def.save_to_path(&path);
            def
        }
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, HardwareError> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| HardwareError::Io(format!("Could not read config file: {e}")))?;
        let mut config: AppConfig = toml::from_str(&content)
            .map_err(|e| HardwareError::Parse(format!("Failed to parse TOML configuration: {e}")))?;

        // Config version migration
        if config.version < CURRENT_CONFIG_VERSION {
            info!("Migrating config schema from version {} to {}", config.version, CURRENT_CONFIG_VERSION);
            config.version = CURRENT_CONFIG_VERSION;
            let _ = config.save_to_path(path.as_ref());
        }

        Ok(config)
    }

    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), HardwareError> {
        let p = path.as_ref();
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| HardwareError::Io(format!("Could not create config dir: {e}")))?;
        }
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| HardwareError::Parse(format!("Failed to serialize config: {e}")))?;
        
        // Write atomically via temporary file
        let tmp_path = p.with_extension("tmp");
        fs::write(&tmp_path, serialized)
            .map_err(|e| HardwareError::Io(format!("Could not write tmp config file: {e}")))?;
        fs::rename(&tmp_path, p)
            .map_err(|e| HardwareError::Io(format!("Could not commit config file: {e}")))?;
        
        Ok(())
    }
}
