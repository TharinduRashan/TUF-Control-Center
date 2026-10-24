pub mod backend;
pub mod capabilities;
pub mod config;
pub mod error;
pub mod linux;
pub mod models;
pub mod mock;
pub mod telemetry;

pub use backend::HardwareBackend;
pub use capabilities::HardwareCapabilities;
pub use config::AppConfig;
pub use error::HardwareError;
pub use linux::LinuxHardwareBackend;
pub use mock::MockHardwareBackend;
pub use models::*;
pub use telemetry::LinuxTelemetryReader;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_mode_mapping() {
        assert_eq!(PerformanceMode::Balanced.to_throttle_thermal_policy(), 0);
        assert_eq!(PerformanceMode::Turbo.to_throttle_thermal_policy(), 1);
        assert_eq!(PerformanceMode::Silent.to_throttle_thermal_policy(), 2);

        assert_eq!(
            PerformanceMode::from_throttle_thermal_policy(0).unwrap(),
            PerformanceMode::Balanced
        );
        assert_eq!(
            PerformanceMode::from_throttle_thermal_policy(1).unwrap(),
            PerformanceMode::Turbo
        );
        assert_eq!(
            PerformanceMode::from_throttle_thermal_policy(2).unwrap(),
            PerformanceMode::Silent
        );
        assert!(PerformanceMode::from_throttle_thermal_policy(5).is_err());
    }

    #[test]
    fn test_battery_limits() {
        assert_eq!(BatteryLimit::from_pct(60).unwrap(), BatteryLimit::MaximumLifespan);
        assert_eq!(BatteryLimit::from_pct(80).unwrap(), BatteryLimit::Balanced);
        assert_eq!(BatteryLimit::from_pct(100).unwrap(), BatteryLimit::Full);
        assert!(BatteryLimit::from_pct(75).is_err());
        assert!(BatteryLimit::from_pct(0).is_err());
    }

    #[test]
    fn test_color_hex_parsing() {
        let orange = RgbColor::from_hex("#FF8000").unwrap();
        assert_eq!(orange.r, 255);
        assert_eq!(orange.g, 128);
        assert_eq!(orange.b, 0);
        assert_eq!(orange.to_hex(), "#FF8000");

        assert!(RgbColor::from_hex("invalid").is_err());
        assert!(RgbColor::from_hex("#FFF").is_err());
    }

    #[test]
    fn test_fan_curve_validation() {
        // Valid curve
        let valid = FanCurve {
            points: vec![
                FanCurvePoint { temp_c: 40, speed_pct: 20 },
                FanCurvePoint { temp_c: 55, speed_pct: 40 },
                FanCurvePoint { temp_c: 70, speed_pct: 65 },
                FanCurvePoint { temp_c: 85, speed_pct: 90 },
            ],
        };
        assert!(valid.validate().is_ok());

        // Non-monotonic temperature
        let non_monotonic = FanCurve {
            points: vec![
                FanCurvePoint { temp_c: 40, speed_pct: 20 },
                FanCurvePoint { temp_c: 35, speed_pct: 40 },
                FanCurvePoint { temp_c: 70, speed_pct: 65 },
                FanCurvePoint { temp_c: 85, speed_pct: 90 },
            ],
        };
        assert!(non_monotonic.validate().is_err());

        // Safety violation: 0% speed at high temperature
        let unsafe_curve = FanCurve {
            points: vec![
                FanCurvePoint { temp_c: 40, speed_pct: 20 },
                FanCurvePoint { temp_c: 55, speed_pct: 30 },
                FanCurvePoint { temp_c: 75, speed_pct: 0 },
                FanCurvePoint { temp_c: 85, speed_pct: 90 },
            ],
        };
        assert!(unsafe_curve.validate().is_err());
    }

    #[test]
    fn test_mock_backend_operations() {
        let mut mock = MockHardwareBackend::new();
        let caps = mock.detect_capabilities();
        assert_eq!(caps.model_name, "ASUS TUF Gaming A15 FA506NC_FA506NC");
        assert!(!caps.has_hardware_mux);

        // Telemetry read
        let telemetry = mock.read_telemetry().unwrap();
        assert_eq!(telemetry.cpu.name, "AMD Ryzen 5 7535HS");
        assert!(telemetry.cpu.temp_c > 30.0 && telemetry.cpu.temp_c < 100.0);

        // Mode change
        mock.set_performance_mode(PerformanceMode::Turbo).unwrap();
        assert_eq!(mock.get_performance_mode(), PerformanceMode::Turbo);

        // Battery limit change
        mock.set_battery_limit(BatteryLimit::MaximumLifespan).unwrap();
        assert_eq!(mock.get_battery_limit(), BatteryLimit::MaximumLifespan);

        // Hardware MUX rejection test on FA506NC
        let res = mock.set_gpu_mode(GpuMode::Dedicated);
        assert!(res.is_err());

        // Error injection test
        mock.inject_error(Some(HardwareError::PermissionDenied("test".into())));
        assert!(mock.read_telemetry().is_err());
    }

    #[test]
    fn test_config_serialization() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let mut cfg = AppConfig::default();
        cfg.performance_mode = PerformanceMode::Turbo;
        cfg.battery_limit = BatteryLimit::MaximumLifespan;
        cfg.save_to_path(&config_path).unwrap();

        let loaded = AppConfig::load_from_path(&config_path).unwrap();
        assert_eq!(loaded.performance_mode, PerformanceMode::Turbo);
        assert_eq!(loaded.battery_limit, BatteryLimit::MaximumLifespan);
    }
}
