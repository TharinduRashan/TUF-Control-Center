use gtk4::{Label, ScrolledWindow};
use adw::prelude::*;
use adw::{ActionRow, PreferencesGroup, PreferencesPage};
use tuf_core::models::SystemTelemetry;
use crate::app::BackendHandle;

#[derive(Clone)]
pub struct FansPage {
    pub container: ScrolledWindow,
    cpu_rpm_label: Label,
    gpu_rpm_label: Label,
    status_label: Label,
}

impl FansPage {
    pub fn new(_backend: BackendHandle) -> Self {
        let page = PreferencesPage::new();

        let group = PreferencesGroup::builder()
            .title("Dual Fan Telemetry & Curves")
            .description("Real-time cooling performance and acoustic profiles")
            .build();

        let status_row = ActionRow::builder()
            .title("Cooling System Status")
            .subtitle("Thermal profile regulating fan speeds")
            .build();
        let status_label = Label::builder().label("Reading...").build();
        status_row.add_suffix(&status_label);
        group.add(&status_row);

        let cpu_fan_row = ActionRow::builder()
            .title("CPU Blower Fan")
            .subtitle("Dynamic RPM governed by AMD Ryzen Tctl temperature")
            .build();
        let cpu_rpm_label = Label::builder().label("0 RPM").build();
        cpu_fan_row.add_suffix(&cpu_rpm_label);
        group.add(&cpu_fan_row);

        let gpu_fan_row = ActionRow::builder()
            .title("GPU Blower Fan")
            .subtitle("Dynamic RPM governed by NVIDIA GA107 hotspot temperature")
            .build();
        let gpu_rpm_label = Label::builder().label("0 RPM").build();
        gpu_fan_row.add_suffix(&gpu_rpm_label);
        group.add(&gpu_fan_row);

        page.add(&group);

        let curves_group = PreferencesGroup::builder()
            .title("Firmware Curve Mappings")
            .description("On ASUS TUF models, fan curves are controlled by Embedded Controller firmware modes")
            .build();

        let silent_curve = ActionRow::builder()
            .title("Silent Curve")
            .subtitle("Fans off up to 55°C; acoustic target capped under 32 dBA")
            .build();
        curves_group.add(&silent_curve);

        let balanced_curve = ActionRow::builder()
            .title("Balanced Curve")
            .subtitle("Progressive linear scaling up to 4200 RPM at 85°C")
            .build();
        curves_group.add(&balanced_curve);

        let turbo_curve = ActionRow::builder()
            .title("Turbo Overboost Curve")
            .subtitle("Aggressive ramp-up reaching peak 5000+ RPM under sustained load")
            .build();
        curves_group.add(&turbo_curve);

        page.add(&curves_group);

        let container = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&page)
            .build();

        Self {
            container,
            cpu_rpm_label,
            gpu_rpm_label,
            status_label,
        }
    }

    pub fn update_telemetry(&self, telemetry: &SystemTelemetry) {
        self.cpu_rpm_label.set_text(&format!("{} RPM ({:.1}°C)", telemetry.fans.cpu_fan_rpm, telemetry.cpu.temp_c));
        
        let gpu_temp_str = telemetry.gpu.temp_c
            .map(|t| format!(" ({:.1}°C)", t))
            .unwrap_or_else(|| " (Suspended)".into());
        self.gpu_rpm_label.set_text(&format!("{} RPM{}", telemetry.fans.gpu_fan_rpm, gpu_temp_str));

        self.status_label.set_text(&format!("Mode: {} (EC Regulated)", telemetry.active_mode));
    }
}
