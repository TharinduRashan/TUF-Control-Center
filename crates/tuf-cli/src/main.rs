use clap::{Parser, Subcommand};
use tuf_core::models::{BatteryLimit, GpuMode, PerformanceMode, RgbColor, RgbMode, RgbSpeed};
use tuf_core::{HardwareBackend, LinuxHardwareBackend, MockHardwareBackend};

#[derive(Parser, Debug)]
#[command(
    name = "tuf-cli",
    author = "TUF Control Center Contributors",
    version = tuf_core::version(),
    about = "Command line interface for ASUS TUF Control Center"
)]
struct Cli {
    /// Enable hardware simulation mode
    #[arg(long, env = "TUF_SIMULATE")]
    simulate: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// View real-time system status and hardware telemetry
    Status {
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Switch operating mode (silent, balanced, turbo)
    Profile {
        #[arg(value_parser = parse_profile)]
        mode: PerformanceMode,
    },
    /// Set maximum battery charge threshold (60, 80, 100)
    BatteryLimit {
        #[arg(value_parser = parse_battery_limit)]
        limit: BatteryLimit,
    },
    /// Switch GPU mode (hybrid, integrated)
    Gpu {
        #[arg(value_parser = parse_gpu_mode)]
        mode: GpuMode,
    },
    /// Configure keyboard RGB lighting
    Rgb {
        /// Animation mode: static, breathing, strobing, colorcycle
        #[arg(long, default_value = "static", value_parser = parse_rgb_mode)]
        mode: RgbMode,
        /// Hex color (e.g. #FF8000)
        #[arg(long, default_value = "#FF8000", value_parser = parse_hex_color)]
        color: RgbColor,
        /// Animation speed: slow, normal, fast
        #[arg(long, default_value = "normal", value_parser = parse_rgb_speed)]
        speed: RgbSpeed,
    },
    /// Set keyboard backlight brightness (0 to 3)
    Brightness {
        #[arg(value_parser = clap::value_parser!(u8).range(0..=3))]
        level: u8,
    },
}

fn parse_profile(s: &str) -> Result<PerformanceMode, String> {
    s.parse::<PerformanceMode>().map_err(|e| e.to_string())
}

fn parse_battery_limit(s: &str) -> Result<BatteryLimit, String> {
    let val: u8 = s
        .parse()
        .map_err(|_| "Must be a number (60, 80, or 100)".to_string())?;
    BatteryLimit::from_pct(val).map_err(|e| e.to_string())
}

fn parse_gpu_mode(s: &str) -> Result<GpuMode, String> {
    s.parse::<GpuMode>().map_err(|e| e.to_string())
}

fn parse_rgb_mode(s: &str) -> Result<RgbMode, String> {
    match s.trim().to_lowercase().as_str() {
        "static" => Ok(RgbMode::Static),
        "breathing" => Ok(RgbMode::Breathing),
        "strobing" => Ok(RgbMode::Strobing),
        "colorcycle" => Ok(RgbMode::ColorCycle),
        other => Err(format!(
            "Invalid RGB mode '{other}'. Supported modes: static, breathing, strobing, colorcycle"
        )),
    }
}

fn parse_rgb_speed(s: &str) -> Result<RgbSpeed, String> {
    match s.trim().to_lowercase().as_str() {
        "slow" => Ok(RgbSpeed::Slow),
        "normal" => Ok(RgbSpeed::Normal),
        "fast" => Ok(RgbSpeed::Fast),
        other => Err(format!(
            "Invalid RGB speed '{other}'. Supported speeds: slow, normal, fast"
        )),
    }
}

fn parse_hex_color(s: &str) -> Result<RgbColor, String> {
    RgbColor::from_hex(s).map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.simulate {
        let mut mock = MockHardwareBackend::new();
        handle_command(cli.command, &mut mock).await?;
    } else {
        let mut backend = LinuxHardwareBackend::new();
        handle_command(cli.command, &mut backend).await?;
    }

    Ok(())
}

async fn handle_command<B: HardwareBackend>(
    cmd: Commands,
    backend: &mut B,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::Status { json } => {
            let caps = backend.detect_capabilities();
            let telemetry = backend.read_telemetry()?;

            if json {
                let status_json = serde_json::json!({
                    "capabilities": caps,
                    "telemetry": telemetry,
                });
                println!("{}", serde_json::to_string_pretty(&status_json)?);
            } else {
                println!("==================================================");
                println!("  TUF Control Center Hardware Status");
                println!("==================================================");
                println!("  Model           : {}", caps.model_name);
                println!("  Mode            : {}", telemetry.active_mode);
                println!("  GPU Mode        : {}", telemetry.active_gpu_mode);
                println!(
                    "  CPU             : {:.1}% | {:.1}°C",
                    telemetry.cpu.usage_pct, telemetry.cpu.temp_c
                );
                if telemetry.gpu.is_active {
                    let usage = telemetry.gpu.usage_pct.unwrap_or(0.0);
                    let temp = telemetry.gpu.temp_c.unwrap_or(0.0);
                    let power = telemetry.gpu.power_w.unwrap_or(0.0);
                    println!(
                        "  dGPU (NVIDIA)   : {:.1}% | {:.1}°C ({:.1} W)",
                        usage, temp, power
                    );
                } else {
                    println!("  dGPU (NVIDIA)   : Suspended / D3cold (Saving power)");
                }
                println!(
                    "  Memory          : {} MB / {} MB ({:.1}%)",
                    telemetry.memory.used_mb, telemetry.memory.total_mb, telemetry.memory.usage_pct
                );
                println!(
                    "  Fans            : CPU {} RPM | GPU {} RPM",
                    telemetry.fans.cpu_fan_rpm, telemetry.fans.gpu_fan_rpm
                );
                println!(
                    "  Battery         : {}% [{}] (Limit: {})",
                    telemetry.battery.percentage,
                    telemetry.battery.state,
                    telemetry.battery.current_limit
                );
                println!("==================================================");
            }
        }
        Commands::Profile { mode } => {
            backend.set_performance_mode(mode)?;
            println!("Operating profile set to: {}", mode);
        }
        Commands::BatteryLimit { limit } => {
            backend.set_battery_limit(limit)?;
            println!("Battery charge limit set to: {}", limit);
        }
        Commands::Gpu { mode } => {
            backend.set_gpu_mode(mode)?;
            println!("GPU mode set to: {}", mode);
        }
        Commands::Rgb { mode, color, speed } => {
            let config = tuf_core::models::RgbConfig {
                mode,
                color,
                speed,
                brightness: 3,
            };
            backend.set_keyboard_rgb(&config)?;
            println!(
                "RGB configured: mode={}, color={}, speed={:?}",
                config.mode.display_name(),
                color.to_hex(),
                speed
            );
        }
        Commands::Brightness { level } => {
            backend.set_keyboard_brightness(level)?;
            println!("Keyboard backlight brightness set to: {}", level);
        }
    }
    Ok(())
}
