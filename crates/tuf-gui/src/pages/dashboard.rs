use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup};
use tuf_core::models::{BatteryLimit, PerformanceMode, SystemTelemetry};
use crate::app::BackendHandle;
use crate::widgets::TelemetryCard;

#[derive(Clone)]
pub struct DashboardPage {
    pub container: ScrolledWindow,
    cpu_card: TelemetryCard,
    gpu_card: TelemetryCard,
    mem_card: TelemetryCard,
    fans_card: TelemetryCard,
    bat_card: TelemetryCard,
    status_label: Label,
    btn_silent: Button,
    btn_balanced: Button,
    btn_turbo: Button,
    btn_limit_60: Button,
    btn_limit_80: Button,
    btn_limit_100: Button,
}

impl DashboardPage {
    pub fn new(backend: BackendHandle) -> Self {
        let content = GtkBox::new(Orientation::Vertical, 16);
        content.set_margin_start(16);
        content.set_margin_end(16);
        content.set_margin_top(16);
        content.set_margin_bottom(16);

        // Header status summary
        let status_box = GtkBox::new(Orientation::Horizontal, 12);
        let status_label = Label::builder()
            .label("Connecting to hardware telemetry...")
            .css_classes(["heading"])
            .xalign(0.0)
            .hexpand(true)
            .build();
        status_box.append(&status_label);
        content.append(&status_box);

        // Quick Mode Switcher Group
        let mode_group = PreferencesGroup::builder()
            .title("Operating Profile")
            .description("Hardware thermal throttle policy and power ceiling")
            .build();

        let mode_row = ActionRow::builder()
            .title("Active Performance Mode")
            .subtitle("Changes fan curves, CPU TDP, and AMD boost profile")
            .build();

        let mode_btn_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_silent = Button::with_label("Silent");
        let btn_balanced = Button::with_label("Balanced");
        let btn_turbo = Button::with_label("Turbo");

        btn_silent.add_css_class("suggested-action");
        btn_balanced.add_css_class("suggested-action");
        btn_turbo.add_css_class("suggested-action");

        mode_btn_box.append(&btn_silent);
        mode_btn_box.append(&btn_balanced);
        mode_btn_box.append(&btn_turbo);
        mode_row.add_suffix(&mode_btn_box);
        mode_group.add(&mode_row);
        content.append(&mode_group);

        // Telemetry Grid
        let grid = gtk4::Grid::builder()
            .column_spacing(12)
            .row_spacing(12)
            .column_homogeneous(true)
            .build();

        let cpu_card = TelemetryCard::new("CPU — AMD Ryzen 5", "-- °C", "0.0% Utilization | 0 MHz");
        let gpu_card = TelemetryCard::new("GPU — NVIDIA RTX 3050", "-- °C", "0.0% Load | 0W");
        let mem_card = TelemetryCard::new("System Memory", "0 / 0 MB", "0.0% Used");
        let fans_card = TelemetryCard::new("Dual Cooling Fans", "CPU: 0 RPM", "GPU: 0 RPM");
        let bat_card = TelemetryCard::new("Battery & Power", "--%", "State: -- | Limit: --");

        grid.attach(&cpu_card.container, 0, 0, 1, 1);
        grid.attach(&gpu_card.container, 1, 0, 1, 1);
        grid.attach(&mem_card.container, 0, 1, 1, 1);
        grid.attach(&fans_card.container, 1, 1, 1, 1);
        grid.attach(&bat_card.container, 0, 2, 2, 1);
        content.append(&grid);

        // Quick Battery Limit Group
        let bat_group = PreferencesGroup::builder()
            .title("Battery Charge Threshold")
            .description("Embedded controller charge limit to preserve battery lifespan")
            .build();

        let bat_row = ActionRow::builder()
            .title("Max Charge Limit")
            .subtitle("Configures AC charge cutoff register")
            .build();

        let bat_btn_box = GtkBox::new(Orientation::Horizontal, 6);
        let btn_limit_60 = Button::with_label("60% Lifespan");
        let btn_limit_80 = Button::with_label("80% Longevity");
        let btn_limit_100 = Button::with_label("100% Full");

        bat_btn_box.append(&btn_limit_60);
        bat_btn_box.append(&btn_limit_80);
        bat_btn_box.append(&btn_limit_100);
        bat_row.add_suffix(&bat_btn_box);
        bat_group.add(&bat_row);
        content.append(&bat_group);

        // Connect button signals
        {
            let b = backend.clone();
            btn_silent.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Silent);
                }
            });
        }
        {
            let b = backend.clone();
            btn_balanced.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Balanced);
                }
            });
        }
        {
            let b = backend.clone();
            btn_turbo.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_performance_mode(PerformanceMode::Turbo);
                }
            });
        }
        {
            let b = backend.clone();
            btn_limit_60.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::MaximumLifespan);
                }
            });
        }
        {
            let b = backend.clone();
            btn_limit_80.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::Balanced);
                }
            });
        }
        {
            let b = backend.clone();
            btn_limit_100.connect_clicked(move |_| {
                if let Ok(mut lock) = b.lock() {
                    let _ = lock.set_battery_limit(BatteryLimit::Full);
                }
            });
        }

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&content)
            .build();

        Self {
            container,
            cpu_card,
            gpu_card,
            mem_card,
            fans_card,
            bat_card,
            status_label,
            btn_silent,
            btn_balanced,
            btn_turbo,
            btn_limit_60,
            btn_limit_80,
            btn_limit_100,
        }
    }

    pub fn update_telemetry(&self, telemetry: &SystemTelemetry) {
        self.status_label.set_text(&format!(
            "Mode: {}  •  GPU: {}  •  Power: {}",
            telemetry.active_mode,
            telemetry.active_gpu_mode,
            if telemetry.battery.ac_connected { "AC Power Connected" } else { "Running on Battery" }
        ));

        // CPU Card
        let cpu_val = format!("{:.1}°C", telemetry.cpu.temp_c);
        let cpu_sub = format!(
            "{:.1}% Load  |  {:.0} MHz{}",
            telemetry.cpu.usage_pct,
            telemetry.cpu.freq_mhz,
            telemetry.cpu.power_w.map(|w| format!("  |  {:.1}W", w)).unwrap_or_default()
        );
        self.cpu_card.update(&cpu_val, &cpu_sub, (telemetry.cpu.usage_pct / 100.0) as f64);

        // GPU Card
        let (gpu_val, gpu_sub, gpu_frac) = if telemetry.gpu.is_active {
            let temp = telemetry.gpu.temp_c.unwrap_or(0.0);
            let load = telemetry.gpu.usage_pct.unwrap_or(0.0);
            let power = telemetry.gpu.power_w.unwrap_or(0.0);
            (
                format!("{:.1}°C", temp),
                format!("{:.1}% Load  |  {:.1}W  |  RTD3 Active", load, power),
                (load / 100.0) as f64,
            )
        } else {
            (
                "Suspended (0W)".into(),
                "PCI Runtime D3cold  |  Sleeping".into(),
                0.0,
            )
        };
        self.gpu_card.update(&gpu_val, &gpu_sub, gpu_frac);

        // Memory Card
        let mem_val = format!("{} / {} MB", telemetry.memory.used_mb, telemetry.memory.total_mb);
        let mem_sub = format!("{:.1}% Utilized", telemetry.memory.usage_pct);
        self.mem_card.update(&mem_val, &mem_sub, (telemetry.memory.usage_pct / 100.0) as f64);

        // Fans Card
        let fans_val = format!("CPU: {} RPM", telemetry.fans.cpu_fan_rpm);
        let fans_sub = format!("GPU: {} RPM", telemetry.fans.gpu_fan_rpm);
        let max_rpm = 5000.0;
        let fan_frac = (telemetry.fans.cpu_fan_rpm as f64 / max_rpm).clamp(0.0, 1.0);
        self.fans_card.update(&fans_val, &fans_sub, fan_frac);

        // Battery Card
        let bat_val = format!("{}% — {}", telemetry.battery.percentage, telemetry.battery.state);
        let bat_sub = format!(
            "Threshold: {}  |  Health: {:.1}%  |  Cycles: {}",
            telemetry.battery.current_limit,
            telemetry.battery.health_pct.unwrap_or(100.0),
            telemetry.battery.cycle_count.unwrap_or(0)
        );
        self.bat_card.update(&bat_val, &bat_sub, telemetry.battery.percentage as f64 / 100.0);

        // Highlight active buttons
        self.update_active_buttons(telemetry.active_mode, telemetry.battery.current_limit);
    }

    fn update_active_buttons(&self, mode: PerformanceMode, limit: BatteryLimit) {
        let set_active = |btn: &Button, active: bool| {
            if active {
                btn.add_css_class("suggested-action");
            } else {
                btn.remove_css_class("suggested-action");
            }
        };

        set_active(&self.btn_silent, mode == PerformanceMode::Silent);
        set_active(&self.btn_balanced, mode == PerformanceMode::Balanced);
        set_active(&self.btn_turbo, mode == PerformanceMode::Turbo);

        set_active(&self.btn_limit_60, limit == BatteryLimit::MaximumLifespan);
        set_active(&self.btn_limit_80, limit == BatteryLimit::Balanced);
        set_active(&self.btn_limit_100, limit == BatteryLimit::Full);
    }
}
